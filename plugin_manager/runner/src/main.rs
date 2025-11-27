use abi_stable::library::lib_header_from_path;
use interface::{PluginRef, PluginRoot_Ref};

use abi_stable::std_types::{RResult, RString, Tuple2};
use std::io::{Read, Write};
use std::os::fd::FromRawFd;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::thread;

struct LoadedPlugin {
    root: PluginRoot_Ref,
    path: PathBuf,
    functions: RString,
    name: RString,
}

fn spawn_start_if_exists(plugin: &mut LoadedPlugin) {
    let name = plugin.path.display().to_string();

    let plugin_ref: &PluginRef = &plugin.root.plugin();
    let init_fn = match plugin_ref.init() {
        Some(f) => f,
        None => {
            println!("[RUNNER {name}] ERROR: init() is None");
            return;
        }
    };

    let init_result = init_fn();

    match init_result {
        RResult::ROk(vec) => {
            println!(
                "[RUNNER {name}] init() returned OK with {} entries:",
                vec.len()
            );

            for Tuple2(key, value) in vec {
                if key == "function" {
                    plugin.functions = value.clone();
                }
                if key == "name" {
                    plugin.name = value.clone();
                }
                println!("    {} => {}", key.as_str(), value.as_str());
            }
        }

        RResult::RErr(err_msg) => {
            println!("[RUNNER {name}] init() ERROR: {}", err_msg.as_str());
        }
    }
}

fn load_plugin(path: &Path) -> Result<LoadedPlugin, String> {
    let header = lib_header_from_path(path).map_err(|e| format!("header load failed: {e}"))?;

    let root: PluginRoot_Ref = header
        .init_root_module::<PluginRoot_Ref>()
        .map_err(|e| format!("init root failed: {e}"))?;

    Ok(LoadedPlugin {
        root,
        path: path.to_path_buf(),
        functions: "".into(),
        name: "".into(),
    })
}

fn is_shared_library(path: &Path) -> bool {
    path.is_file() && path.extension().map_or(false, |ext| ext == "so")
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("[ERROR] Bad runner usage");
        exit(1);
    }

    let so_path = Path::new(&args[1]);
    let tmp_name = so_path.display().to_string();
    let name = tmp_name.rsplit('/').next().unwrap().to_string();

    if is_shared_library(so_path) == false {
        println!(
            "[RUNNER](ERROR): {} is not a compatible plugin.",
            so_path.display()
        );
        exit(1);
    }
    let mut plugin = load_plugin(so_path).unwrap_or_else(|e| {
        eprintln!("[ERROR] Failed to load plugin: {e}");
        exit(1);
    });

    spawn_start_if_exists(&mut plugin);

    let fd = 3;
    let mut sock = unsafe { std::os::unix::net::UnixStream::from_raw_fd(fd) };

    thread::spawn(move || {
        let mut buf = [0u8; 1024];
        loop {
            match sock.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    let msg = &buf[..n];
                    // print!("[PLUGIN {name}] Received: {}", String::from_utf8_lossy(msg));

                    let rmsg = RString::from(String::from_utf8(msg.into()).unwrap());

                    if rmsg == "6ebfd31e78daab8796928c04cdc866deba88c7135d51ddc473288ac49949b646" {
                        let msg = format!("{}|{}", plugin.functions, plugin.name);
                        sock.write_all(msg.as_bytes()).expect("failed to write");
                    } // "info_init123456789"
                     else {
                         let plugin_ref: &PluginRef = &plugin.root.plugin();
                         let handle_fn: extern "C" fn(RString) -> RString =
                             plugin_ref.handle_message().unwrap();
                         let response: RString = handle_fn(rmsg);

                         let _ = sock.write_all(response.into_bytes().as_slice());
                     }
                }
                Err(e) => {
                    eprintln!("[PLUGIN {name}](ERROR) reading socket: {e}");
                    break;
                }
            }
        }
    });

    loop {
        thread::park();
    }
}
