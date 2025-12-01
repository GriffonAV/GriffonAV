
use nix::libc;
use nix::sys::socket::{AddressFamily, SockFlag, SockType, socketpair};
use std::fs::read_dir;
use std::io::{self, Read, Write};
use std::os::fd::{AsRawFd, FromRawFd, IntoRawFd};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};

static RUNNER_BINARY: &str = "../../target/debug/runner";

#[derive(Debug)]
struct RunningPlugin {
    process: Child,
    fd: std::os::unix::net::UnixStream,
    pub plugin_info: PluginInfo,
}

#[derive(Debug, Clone)]
pub struct PluginInfo {
    pub pid: u32,
    pub name: String,
    pub path: PathBuf,
    pub functions: Vec<String>,
}

#[derive(Debug, PartialEq, PartialOrd, Copy, Clone)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub struct PluginManager {
    pub plugins_dir: PathBuf,
    plugins_list: Vec<RunningPlugin>,
    pub log_level: LogLevel,
}

impl PluginManager {
    pub fn new<P: AsRef<Path>>(dir: P, log_level: LogLevel) -> Self {
        Self {
            plugins_dir: dir.as_ref().to_path_buf(),
            plugins_list: Vec::new(),
            log_level
        }
    }

    fn log(&self, level: LogLevel, msg: &str) {
        if level >= self.log_level {
            match level {
                LogLevel::Debug => println!("[PLUGIN_MANAGER](DEBUG) {}", msg),
                LogLevel::Info  => println!("[PLUGIN_MANAGER](INFO) {}", msg),
                LogLevel::Warn  => println!("[PLUGIN_MANAGER](WARN) {}", msg),
                LogLevel::Error => eprintln!("[PLUGIN_MANAGER](ERROR) {}", msg),
            }
        }
    }

    pub fn list_plugins(&self) -> Vec<PluginInfo> {
        let mut out = Vec::new();

        for plugin in &self.plugins_list {
            out.push(plugin.plugin_info.clone());
        }

        out
    }


    pub fn scan_dir(&mut self) {
        let mut current_paths = Vec::new();

        for entry in read_dir(&self.plugins_dir).expect("Bad plugin directory") {
            let path = entry.unwrap().path();
            if Self::is_shared_library(&path) {
                current_paths.push(path.clone());
                self.check_plugin(&path);
            }
        }

        let mut i = 0;
        while i < self.plugins_list.len() {
            if !current_paths.contains(&self.plugins_list[i].plugin_info.path) {
                self.remove_plugin_at(i);
            } else {
                i += 1;
            }
        }
    }

    pub fn restart_plugin(&mut self, pid: u32) {
        if let Some(plugin) = self.plugins_list.iter().find(|p| p.process.id() == pid) {
            let path = plugin.plugin_info.path.clone();
            self.kill_plugin(pid);
            self.check_plugin(&path);
            self.log(LogLevel::Debug, &format!("Plugin PID {pid} restarted"));
        } else {
            self.log(LogLevel::Error, &format!("No plugin found with PID {pid}"));
        }
    }

    pub fn kill_plugin(&mut self, pid: u32) {
        if let Some(pos) = self.plugins_list.iter().position(|p| p.process.id() == pid) {
            let mut plugin = self.plugins_list.remove(pos);

            match plugin.process.kill() {
                Ok(_) => self.log(LogLevel::Debug, &format!("Plugin PID {pid} killed")),
                Err(e) =>
                    self.log(LogLevel::Error, &format!("Failed to kill plugin PID {pid}: {e}")),
            }
        } else {
            self.log(LogLevel::Warn, &format!("No plugin found with PID {pid}"));
        }
    }

    pub fn send_msg(&mut self, pid: u32, msg: &str) {
        if let Some(plugin) = self.plugins_list.iter_mut().find(|p| p.process.id() == pid) {
            if let Err(e) = plugin.fd.write_all(msg.as_bytes()) {
                self.log(LogLevel::Error, &format!("Failed to send message to plugin PID: {e}"));
            }
        } else {
            self.log(LogLevel::Warn, &format!("No plugin found with PID {pid}"));
        }
    }

    fn check_plugin(&mut self, path: &Path) {
        let already_running = self.plugins_list.iter().any(|p| p.plugin_info.path == path);
        if already_running {
            self.log(LogLevel::Debug, &format!("Plugin already running {}", path.display()));
            return;
        }

        let running = match Self::launch_runner(self, path) {
            Ok(r) => r,
            Err(msg) => {
                self.log(LogLevel::Error, &format!("Failed to launch runner for plugin {}: {msg}", path.display()));
                return;
            }
        };

        self.log(LogLevel::Debug, &format!("New plugin found {}", path.display()));

        self.plugins_list.push(running);
        let last = self.plugins_list.last_mut().unwrap();
        read_plugin_messages(last, self.log_level);
    }

    fn remove_plugin_at(&mut self, index: usize) {
        let mut plugin = self.plugins_list.remove(index);
        self.log(LogLevel::Debug, &format!("Plugin removed {}", plugin.plugin_info.name));
        if let Err(e) = plugin.process.kill() {
            self.log(LogLevel::Error, &format!("Failed to kill plugin PID : {e}"));
        }
    }


    fn is_shared_library(path: &Path) -> bool {
        path.is_file() && path.extension().map_or(false, |ext| ext == "so")
    }

    fn launch_runner(&self, plugin_path: &Path) -> Result<RunningPlugin, String> {
        let path = plugin_path.display().to_string();
        let tmp_name = plugin_path.display().to_string();
        let name = tmp_name.rsplit('/').next().unwrap().to_string();

        let (core_fd, runner_fd) = socketpair(
            AddressFamily::Unix,
            SockType::SeqPacket,
            None,
            SockFlag::empty(),
        )
            .map_err(|e| format!("socketpair failed: {}", e))?;

        let mut cmd = Command::new(RUNNER_BINARY);
        cmd.arg(path);
        unsafe {
            cmd.pre_exec(move || {
                if libc::dup2(runner_fd.as_raw_fd(), 3) == -1 {
                    return Err(io::Error::last_os_error());
                }
                Ok(())
            });
        }

        let child = cmd.spawn().map_err(|e| format!("failed to spawn runner: {}", e))?;
        let core_stream = unsafe { std::os::unix::net::UnixStream::from_raw_fd(core_fd.into_raw_fd()) };
        let functions = Vec::new();
        let plugininfo = PluginInfo {
            pid: child.id(),
            name,
            path: plugin_path.to_path_buf(),
            functions
        };
        self.log(LogLevel::Info, &format!("Plugin {} ({}) has been started.",plugininfo.name ,plugininfo.pid));
        Ok(RunningPlugin {
            process: child,
            fd: core_stream,
            plugin_info: plugininfo,
        })
    }
}

fn read_plugin_messages(plugin: &mut RunningPlugin, log_level: LogLevel) {
    let mut fd_clone = plugin.fd.try_clone().expect("Failed to clone fd");
    let pid = plugin.process.id();

    fd_clone.write_all(
        b"6ebfd31e78daab8796928c04cdc866deba88c7135d51ddc473288ac49949b646",
    ).expect("failed to write");

    let mut buf = [0u8; 1024];
    let n = fd_clone.read(&mut buf).expect("failed to read");

    let response = String::from_utf8_lossy(&buf[..n]);

    let parts: Vec<&str> = response.split('|').collect();

    let functions_part = parts.get(0).unwrap_or(&"");
    let name_part = parts.get(1).unwrap_or(&"");

    plugin.plugin_info.functions = functions_part
        .split('/')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if !name_part.trim().is_empty() {
        plugin.plugin_info.name = name_part.trim().to_string();
    }
    let name_clone = plugin.plugin_info.name.clone();

    std::thread::spawn(move || {
        let mut buf = [0u8; 1024];
        loop {
            match fd_clone.read(&mut buf) {
                Ok(0) => {
                    if log_level >= LogLevel::Info {
                        println!("[PLUGIN_MANAGER](INFO) Plugin {name_clone} ({pid}) closed connection");
                    }
                    break;
                }
                Ok(n) => {
                    let msg = String::from_utf8_lossy(&buf[..n]);
                    if log_level >= LogLevel::Debug {
                        println!("[PLUGIN_MANAGER](DEBUG) Plugin {name_clone} ({pid}) MSG : {msg}");
                    }
                }
                Err(e) => {
                    if log_level >= LogLevel::Error {
                        println!("[PLUGIN_MANAGER](ERROR) Plugin {name_clone} ({pid}) Read error: {e}");

                    }
                    break;
                }
            }
        }
    });
}