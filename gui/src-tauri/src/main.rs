use plugin_manager::{PluginManager, LogLevel};
use std::sync::Mutex;
use tauri::State;
use serde::Serialize;

static PLUGIN_DIR: &str = "../../target/release";

struct PMState(pub Mutex<PluginManager>);


#[derive(Serialize)]
struct PluginInfo {
    pid: u32,
    name: String,
    functions: Vec<String>,
}

#[tauri::command]
fn list_plugins_cmd(pm: State<PMState>) -> Vec<PluginInfo> {
    let plugins = pm.0.lock().unwrap().list_plugins();
    plugins
        .into_iter()
        .map(|p| PluginInfo {
            pid: p.pid,
            name: p.name.clone(),
            functions: p.functions.clone(),
        })
        .collect()
}

#[tauri::command]
fn list_plugins(pm: State<PMState>) -> Vec<String> {
    let pm = pm.0.lock().unwrap();
    pm.list_plugins()
        .into_iter()
        .map(|p| format!("{}: {}", p.pid, p.name))
        .collect()
}

#[tauri::command]
fn refresh_plugins(pm: State<PMState>) {
    pm.0.lock().unwrap().scan_dir();
}

#[tauri::command]
fn message_plugin(pid: u32, msg: String, pm: State<PMState>) {
    pm.0.lock().unwrap().send_msg(pid, &msg);
}

fn main() {
    let mut pm = PluginManager::new(PLUGIN_DIR, LogLevel::Info);
    pm.scan_dir();

    tauri::Builder::default()
        .manage(PMState(Mutex::new(pm)))
        .invoke_handler(tauri::generate_handler![
            list_plugins,
            refresh_plugins,
            message_plugin,
            list_plugins_cmd       
            ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}