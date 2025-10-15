mod plugin_handle;

use abi_stable::library::lib_header_from_path;
use interface::PluginRoot_Ref;

use std::collections::HashSet;
use std::fs::{DirEntry, read_dir};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::thread;

static PATH_DIR: &str = "target/debug/";

struct RunningPlugin {
    root: PluginRoot_Ref,
    path: PathBuf,
}

fn debug_plugin(label: &str, path: &Path, plugin: &PluginRoot_Ref) { // By ChatGPT sa print les addr des fonction start )
    let abs = path.canonicalize()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| path.display().to_string());
    println!("[{label}] file       : {abs}");

    let start_ptr: *const () = plugin.plugin()
        .start()
        .map(|f| f as *const ())
        .unwrap_or(std::ptr::null());
    println!("[{label}] start fn  : {:p}", start_ptr);

    let plugin_ptr: *const () = {
        let pref = plugin.plugin();
        &pref as *const _ as *const ()
    };
    println!("[{label}] PluginRef : {:p}", plugin_ptr);
}

fn spawn_plugin(plugin: &RunningPlugin) -> Result<(), String> {
    let start_opt = plugin.root.plugin().start();
    let name = plugin.path.display().to_string();
    match start_opt {
        Some(start_fn) => {
            thread::Builder::new()
                .spawn(move || {
                    let _ = std::panic::catch_unwind(|| {
                        let _ = start_fn();
                    }).map_err(|e| eprintln!("[ERROR] ={name}= PANIC: {e:?}"));
                    eprintln!("[ERROR] ={name}= start() returned (thread ending)", );
                })
                .map(|_| ())
                .map_err(|e| format!("spawn failed: {e}"))
        }
        None => Err("No start function in plugin".to_string()),
    }
}

fn is_shared_library(path: &Path) -> bool {
    path.is_file() && path.extension().map_or(false, |ext| ext == "so")
}

fn load_plugin_by_path(path: &Path) -> Result<RunningPlugin, abi_stable::library::LibraryError> {
    let header = lib_header_from_path(path)?;
    let root: PluginRoot_Ref = header.init_root_module::<PluginRoot_Ref>()?;
    Ok(RunningPlugin { root, path: path.to_path_buf() })
}

fn load_and_maybe_run_plugin(
    path: &Path,
    already_started: &mut HashSet<PathBuf>,
    keep_alive: &mut Vec<RunningPlugin>,
) {
    if already_started.contains(path) {
        println!("[DEBUG] (skip) already running plugin: {}", path.display());
        return;
    }

    // println!("[DEBUG] Found plugin: {}", path.display());

    match load_plugin_by_path(path) {
        Ok(plugin) => {
            // debug_plugin(&path.file_name().unwrap().to_string_lossy(), path, &plugin.root);
            match spawn_plugin(&plugin) {
                Ok(()) => {
                    // println!("[DEBUG] plugin {} -> OK", path.display());
                    already_started.insert(path.to_path_buf());
                    keep_alive.push(plugin);
                }
                Err(msg) => eprintln!("[ERROR] plugin {} -> KO : {msg}", path.display()),
            }
        }
        Err(e) => {
            eprintln!("[ERROR] Unable to load \"{}\" error: {e}", path.display());
        }
    }
}

fn scan_plugins_dir(
    dir: &Path,
    already_started: &mut HashSet<PathBuf>,
    keep_alive: &mut Vec<RunningPlugin>,
) {
    println!("[DEBUG] Scanning plugin from {}", dir.display());
    for entry in read_dir(dir)
        .unwrap_or_else(|e| panic!("[ERROR] Bad path, please check dir \"{}\" : {}", PATH_DIR, e))
    {
        let entry: DirEntry = entry.unwrap();
        let path: PathBuf = entry.path();

        if is_shared_library(&path) {
            load_and_maybe_run_plugin(&path, already_started, keep_alive);
        }
    }
}


fn main() {
    let dir = Path::new(PATH_DIR);

    let mut already_started = HashSet::<PathBuf>::new();
    let mut keep_alive = Vec::<RunningPlugin>::new();

    scan_plugins_dir(dir, &mut already_started, &mut keep_alive);

    loop {
        print!("$> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("[ERROR] Read Error");
            continue;
        }

        match input.trim() {
            "info" => {
                println!("Nbr of Plugins: {}", keep_alive.len());
                println!("Plugin list {:?}", already_started);
            }
            "refresh" => {
                scan_plugins_dir(dir, &mut already_started, &mut keep_alive);
            }
            "exit" | "quit" => {
                break;
            }
            "" => {}
            other => {
                println!("Unknow CMD: {}", other);
            }
        }
    }
}
