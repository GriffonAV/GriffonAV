use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock, mpsc};
use std::thread::{self, JoinHandle};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// -------- Public API --------

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl Level {
    pub fn as_str(self) -> &'static str {
        match self {
            Level::Trace => "TRACE",
            Level::Debug => "DEBUG",
            Level::Info  => "INFO",
            Level::Warn  => "WARN",
            Level::Error => "ERROR",
        }
    }
}

#[derive(Debug)]
pub enum LogError {
    Io(io::Error),
    AlreadySet(&'static str),
    ThreadPanic,
}

impl fmt::Display for LogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogError::Io(e) => write!(f, "io error: {e}"),
            LogError::AlreadySet(s) => write!(f, "{s}"),
            LogError::ThreadPanic => write!(f, "logger worker thread panicked"),
        }
    }
}

impl From<io::Error> for LogError {
    fn from(e: io::Error) -> Self { Self::Io(e) }
}

#[derive(Clone)]
pub struct Logger {
    inner: Arc<Inner>,
}

pub struct LoggerBuilder {
    project: String,
    process: Option<String>,
    log_dir: Option<PathBuf>,

    min_level: Level,
    also_console_stderr: bool,

    mode: Mode,

    rotate_size_bytes: u64,
    rotate_interval_ms: u128,
    retain_days: u64,
    retain_files: usize,
}

#[derive(Copy, Clone, Debug)]
pub enum Mode {
    Async,
    Sync,
}

impl LoggerBuilder {
    pub fn new(project: impl Into<String>) -> Self {
        Self {
            project: project.into(),
            process: None,
            log_dir: None,
            min_level: Level::Info,
            also_console_stderr: false,
            mode: Mode::Async, // agreed: async by default
            rotate_size_bytes: 10 * 1024 * 1024,           // 10MB
            rotate_interval_ms: 24 * 60 * 60 * 1000,       // 24h
            retain_days: 30,
            retain_files: 200,
        }
    }

    /// Optional override. If not set, derived from exe name.
    pub fn process_name(mut self, name: impl Into<String>) -> Self {
        self.process = Some(name.into());
        self
    }

    /// Optional override. If not set, uses /var/log/<project>/
    pub fn log_dir(mut self, dir: impl Into<PathBuf>) -> Self {
        self.log_dir = Some(dir.into());
        self
    }

    pub fn min_level(mut self, level: Level) -> Self {
        self.min_level = level;
        self
    }

    pub fn also_console_stderr(mut self, enabled: bool) -> Self {
        self.also_console_stderr = enabled;
        self
    }

    /// Force sync mode (writes directly on caller thread).
    pub fn sync(mut self) -> Self {
        self.mode = Mode::Sync;
        self
    }

    /// Force async mode (queue + writer thread).
    pub fn async_mode(mut self) -> Self {
        self.mode = Mode::Async;
        self
    }

    /// Optional tuning (defaults are safe).
    pub fn rotation(mut self, size_bytes: u64, interval_ms: u128) -> Self {
        self.rotate_size_bytes = size_bytes;
        self.rotate_interval_ms = interval_ms;
        self
    }

    pub fn retention(mut self, days: u64, max_files: usize) -> Self {
        self.retain_days = days;
        self.retain_files = max_files;
        self
    }

    pub fn build(self) -> Result<Logger, LogError> {
        let process = match self.process {
            Some(p) => p,
            None => default_process_name().unwrap_or_else(|| "process".to_string()),
        };

        let dir = self
            .log_dir
            .unwrap_or_else(|| PathBuf::from(format!("/var/log/{}", self.project)));

        fs::create_dir_all(&dir)?; // note: /var/log typically needs permissions

        let base_path = dir.join(format!("{process}.log"));

        let cfg = Config {
            project: self.project,
            process,
            dir,
            base_path,
            min_level: self.min_level,
            also_console_stderr: self.also_console_stderr,
            rotate_size_bytes: self.rotate_size_bytes,
            rotate_interval_ms: self.rotate_interval_ms,
            retain_days: self.retain_days,
            retain_files: self.retain_files,
            mode: self.mode,
        };

        let inner = Inner::new(cfg)?;
        Ok(Logger { inner: Arc::new(inner) })
    }
}

/// Set the process-wide default logger used by `info!`, `warn!`, etc.
pub fn set_default_logger(logger: Logger) -> Result<(), LogError> {
    DEFAULT_LOGGER
        .set(logger)
        .map_err(|_| LogError::AlreadySet("default logger already set"))
}

/// Get the default logger, if set.
pub fn default_logger() -> Option<&'static Logger> {
    DEFAULT_LOGGER.get()
}

impl Logger {
    pub fn enabled(&self, level: Level) -> bool {
        level >= self.inner.cfg.min_level
    }

    /// Log a message (used by macros).
    pub fn log(&self, level: Level, file: &'static str, line: u32, module: &'static str, args: fmt::Arguments<'_>) {
        if !self.enabled(level) { return; }
        let ts_ms = now_ms();
        let pid = std::process::id();
        let tid = format!("{:?}", thread::current().id());

        let line_str = format_logfmt(ts_ms, level, pid, &tid, module, file, line, args);

        self.inner.write_line(line_str);
    }

    /// Flush pending logs.
    pub fn flush(&self) {
        self.inner.flush();
    }

    /// Shutdown background thread (async mode). Safe to call multiple times.
    pub fn shutdown(&self) {
        self.inner.shutdown();
    }

    /// Expose config paths if you need them.
    pub fn base_path(&self) -> &Path {
        &self.inner.cfg.base_path
    }
}

/// -------- Macros --------
/// - Beginner: info!("...")
/// - Advanced: info!(logger: my_logger, "...")
#[macro_export]
macro_rules! log {
    (logger: $logger:expr, $level:expr, $($arg:tt)*) => {{
        $logger.log($level, file!(), line!(), module_path!(), format_args!($($arg)*));
    }};
    ($level:expr, $($arg:tt)*) => {{
        if let Some(lg) = $crate::default_logger() {
            lg.log($level, file!(), line!(), module_path!(), format_args!($($arg)*));
        }
    }};
}

#[macro_export] macro_rules! trace { (logger: $l:expr, $($a:tt)*) => { $crate::log!(logger: $l, $crate::Level::Trace, $($a)*) }; ($($a:tt)*) => { $crate::log!($crate::Level::Trace, $($a)*) }; }
#[macro_export] macro_rules! debug { (logger: $l:expr, $($a:tt)*) => { $crate::log!(logger: $l, $crate::Level::Debug, $($a)*) }; ($($a:tt)*) => { $crate::log!($crate::Level::Debug, $($a)*) }; }
#[macro_export] macro_rules! info  { (logger: $l:expr, $($a:tt)*) => { $crate::log!(logger: $l, $crate::Level::Info , $($a)*) }; ($($a:tt)*) => { $crate::log!($crate::Level::Info , $($a)*) }; }
#[macro_export] macro_rules! warn  { (logger: $l:expr, $($a:tt)*) => { $crate::log!(logger: $l, $crate::Level::Warn , $($a)*) }; ($($a:tt)*) => { $crate::log!($crate::Level::Warn , $($a)*) }; }
#[macro_export] macro_rules! error { (logger: $l:expr, $($a:tt)*) => { $crate::log!(logger: $l, $crate::Level::Error, $($a)*) }; ($($a:tt)*) => { $crate::log!($crate::Level::Error, $($a)*) }; }

/// -------- Internals --------

static DEFAULT_LOGGER: OnceLock<Logger> = OnceLock::new();

#[derive(Clone)]
struct Config {
    project: String,
    process: String,
    dir: PathBuf,
    base_path: PathBuf,

    min_level: Level,
    also_console_stderr: bool,

    rotate_size_bytes: u64,
    rotate_interval_ms: u128,

    retain_days: u64,
    retain_files: usize,

    mode: Mode,
}

struct Inner {
    cfg: Config,
    mode: InnerMode,
}

enum InnerMode {
    Sync {
        state: Mutex<WriterState>,
    },
    Async {
        tx: mpsc::Sender<Cmd>,
        join: Mutex<Option<JoinHandle<()>>>,
    },
}

enum Cmd {
    Line(String),
    Flush(mpsc::Sender<()>),
    Shutdown(mpsc::Sender<()>),
}

struct WriterState {
    file: File,
    start_ms: u128,
    size_bytes: u64,
}

impl Inner {
    fn new(cfg: Config) -> Result<Self, LogError> {
        match cfg.mode {
            Mode::Sync => {
                let st = open_writer_state(&cfg)?;
                // On startup, do a cleanup pass (best effort).
                let _ = cleanup_old(&cfg);
                Ok(Self { cfg, mode: InnerMode::Sync { state: Mutex::new(st) } })
            }
            Mode::Async => {
                let (tx, rx) = mpsc::channel::<Cmd>();
                let cfg_clone = cfg.clone();

                let join = thread::Builder::new()
                    .name(format!("{}-logger", cfg.process))
                    .spawn(move || writer_thread(cfg_clone, rx))
                    .map_err(LogError::Io)?;

                Ok(Self {
                    cfg,
                    mode: InnerMode::Async { tx, join: Mutex::new(Some(join)) },
                })
            }
        }
    }

    fn write_line(&self, line: String) {
        match &self.mode {
            InnerMode::Sync { state } => {
                if let Ok(mut st) = state.lock() {
                    let _ = write_with_rotation(&self.cfg, &mut st, &line);
                }
                if self.cfg.also_console_stderr {
                    let _ = eprintln!("{line}");
                }
            }
            InnerMode::Async { tx, .. } => {
                // Best-effort: if receiver is gone, ignore.
                let _ = tx.send(Cmd::Line(line));
            }
        }
    }

    fn flush(&self) {
        match &self.mode {
            InnerMode::Sync { state } => {
                if let Ok(mut st) = state.lock() {
                    let _ = st.file.flush();
                }
            }
            InnerMode::Async { tx, .. } => {
                let (ack_tx, ack_rx) = mpsc::channel();
                if tx.send(Cmd::Flush(ack_tx)).is_ok() {
                    let _ = ack_rx.recv_timeout(Duration::from_secs(2));
                }
            }
        }
    }

    fn shutdown(&self) {
        if let InnerMode::Async { tx, join } = &self.mode {
            let (ack_tx, ack_rx) = mpsc::channel();
            if tx.send(Cmd::Shutdown(ack_tx)).is_ok() {
                let _ = ack_rx.recv_timeout(Duration::from_secs(2));
            }
            if let Ok(mut opt) = join.lock() {
                if let Some(j) = opt.take() {
                    if j.join().is_err() {
                        // nothing to do here; user can observe missing logs
                    }
                }
            }
        }
    }
}

fn writer_thread(cfg: Config, rx: mpsc::Receiver<Cmd>) {
    let mut st = match open_writer_state(&cfg) {
        Ok(s) => s,
        Err(_) => return,
    };

    let _ = cleanup_old(&cfg);

    while let Ok(cmd) = rx.recv() {
        match cmd {
            Cmd::Line(line) => {
                let _ = write_with_rotation(&cfg, &mut st, &line);
                if cfg.also_console_stderr {
                    let _ = eprintln!("{line}");
                }
            }
            Cmd::Flush(ack) => {
                let _ = st.file.flush();
                let _ = ack.send(());
            }
            Cmd::Shutdown(ack) => {
                let _ = st.file.flush();
                let _ = ack.send(());
                break;
            }
        }
    }
}

fn open_writer_state(cfg: &Config) -> io::Result<WriterState> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&cfg.base_path)?;

    let size_bytes = file.metadata().map(|m| m.len()).unwrap_or(0);
    Ok(WriterState {
        file,
        start_ms: now_ms(), // interval-based rotation from open time
        size_bytes,
    })
}

fn write_with_rotation(cfg: &Config, st: &mut WriterState, line: &str) -> io::Result<()> {
    let now = now_ms();

    let needs_time_rotate = (now.saturating_sub(st.start_ms)) >= cfg.rotate_interval_ms;
    let line_len = (line.as_bytes().len() + 1) as u64; // + '\n'
    let needs_size_rotate = st.size_bytes.saturating_add(line_len) > cfg.rotate_size_bytes;

    if needs_time_rotate || needs_size_rotate {
        rotate(cfg, st, now)?;
        let _ = cleanup_old(cfg); // best effort
    }

    writeln!(st.file, "{line}")?;
    st.size_bytes = st.size_bytes.saturating_add(line_len);
    Ok(())
}

fn rotate(cfg: &Config, st: &mut WriterState, ts_ms: u128) -> io::Result<()> {
    let _ = st.file.flush();

    // Close current file by dropping it, then rename.
    // We reopen after rename.
    let _ = &st.file;

    let rotated_name = format!("{}.{}", cfg.base_path.file_name().unwrap().to_string_lossy(), ts_ms);
    let rotated_path = cfg.dir.join(rotated_name);

    // Rename current base file -> rotated
    let _ = fs::rename(&cfg.base_path, &rotated_path);

    // Reopen fresh base file
    let new_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&cfg.base_path)?;

    st.file = new_file;
    st.start_ms = ts_ms;
    st.size_bytes = 0;
    Ok(())
}

/// Cleanup rotated files by age (days) OR max file count.
/// Rotated files pattern: "<process>.log.<ts_ms>"
fn cleanup_old(cfg: &Config) -> io::Result<()> {
    let cutoff_ms = now_ms().saturating_sub((cfg.retain_days as u128) * 24 * 60 * 60 * 1000);

    let base_name = cfg.base_path.file_name().unwrap().to_string_lossy().to_string();
    let prefix = format!("{base_name}.");

    let mut rotated: Vec<(u128, PathBuf)> = Vec::new();

    for entry in fs::read_dir(&cfg.dir)? {
        let entry = match entry { Ok(e) => e, Err(_) => continue };
        let name = entry.file_name().to_string_lossy().to_string();
        if !name.starts_with(&prefix) { continue; }

        // parse ts_ms suffix
        let ts_str = &name[prefix.len()..];
        if let Ok(ts) = ts_str.parse::<u128>() {
            rotated.push((ts, entry.path()));
        }
    }

    // Remove older than cutoff
    for (ts, path) in rotated.iter() {
        if *ts < cutoff_ms {
            let _ = fs::remove_file(path);
        }
    }

    // Enforce max files (keep newest)
    rotated.sort_by(|a, b| b.0.cmp(&a.0)); // newest first
    if rotated.len() > cfg.retain_files {
        for (_, path) in rotated.iter().skip(cfg.retain_files) {
            let _ = fs::remove_file(path);
        }
    }

    Ok(())
}

/// logfmt-ish escaping:
/// - keys are bare
/// - values that contain spaces/quotes get quoted and escaped
fn escape_value(s: &str) -> String {
    let needs_quotes = s.chars().any(|c| c.is_whitespace() || c == '"' || c == '=');
    if !needs_quotes {
        return s.to_string();
    }
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    for ch in s.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"'  => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            _ => out.push(ch),
        }
    }
    out.push('"');
    out
}

fn format_logfmt(
    ts_ms: u128,
    level: Level,
    pid: u32,
    tid: &str,
    module: &str,
    file: &str,
    line: u32,
    args: fmt::Arguments<'_>,
) -> String {
    let msg = format!("{}", args);

    // Keep it stable & parseable
    format!(
        "ts_ms={} level={} pid={} tid={} module={} file={} line={} msg={}",
        ts_ms,
        level.as_str(),
        pid,
        escape_value(tid),
        escape_value(module),
        escape_value(file),
        line,
        escape_value(&msg),
    )
}

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

fn default_process_name() -> Option<String> {
    std::env::current_exe().ok().and_then(|p| {
        p.file_stem().map(|s| s.to_string_lossy().to_string())
    })
}
