// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serde::Serialize;

#[derive(Serialize, Clone)]
struct PluginInfo {
    pid: u32,
    name: String,
}

#[tauri::command]
fn get_plugins() -> Vec<PluginInfo> {
    vec![
        PluginInfo { pid: 1, name: "Analyser".into() },
        PluginInfo { pid: 2, name: "Logger".into() },
        PluginInfo { pid: 3, name: "Debugger".into() },
    ]
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_plugins])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
