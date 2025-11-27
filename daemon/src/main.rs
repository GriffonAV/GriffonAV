use std::io;
use std::io::Write;
use plugin_manager::{PluginManager, LogLevel};

static PLUGIN_DIR_PATH: &str = "./plugins";

fn main() {
    let mut pm = PluginManager::new(PLUGIN_DIR_PATH, LogLevel::Info);

    pm.scan_dir();
    pm.list_plugins();

    loop {
        print!("$> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("[CORE](ERROR) Read Error");
            continue;
        }
        let trimmed = input.trim();
        if trimmed.is_empty() {
            continue;
        }
        let mut parts = trimmed.splitn(3, ' ');
        let cmd = parts.next().unwrap();

        match cmd {
            "info" => {
                let plugins = pm.list_plugins();
                for plugin in plugins {
                    println!("- PID: {} | NAME: {} | PATH: {} | FUNCTIONS: {:?}", plugin.pid, plugin.name, plugin.path.display(), plugin.functions);
                }
            }
            "refresh" => {
                pm.scan_dir();
            }
            "exit" | "quit" => {
                break;
            }
            "restart" => {
                let pid_str = parts.next();

                if pid_str.is_none() {
                    println!("[CORE](INPUT ERROR) Usage: restart <PID>");
                    continue;
                }

                let pid: u32 = match pid_str.unwrap().parse() {
                    Ok(p) => p,
                    Err(_) => {
                        println!("[CORE](INPUT ERROR) Invalid PID: {pid_str:?}");
                        continue;
                    }
                };
                pm.restart_plugin(pid);
            }
            "kill" => {
                let pid_str = parts.next();

                if pid_str.is_none() {
                    println!("[CORE](INPUT ERROR) Usage: kill <PID>");
                    continue;
                }

                let pid: u32 = match pid_str.unwrap().parse() {
                    Ok(p) => p,
                    Err(_) => {
                        println!("[CORE](INPUT ERROR) Invalid PID: {pid_str:?}");
                        continue;
                    }
                };
                pm.kill_plugin(pid);
            }

            "msg" => {
                let pid_str = parts.next();
                let msg = parts.next();

                if pid_str.is_none() || msg.is_none() {
                    println!("[CORE](INPUT ERROR) Usage: msg <PID> <message>");
                    continue;
                }

                let pid: u32 = match pid_str.unwrap().parse() {
                    Ok(p) => p,
                    Err(_) => {
                        println!("[CORE](INPUT ERROR) Invalid PID: {:?}", pid_str);
                        continue;
                    }
                };

                let message = msg.unwrap();

                pm.send_msg(pid, message);
            }

            "" => {}
            other => {
                println!("Unknow CMD: {}", other);
            }
        }
    }
}