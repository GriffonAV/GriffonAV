// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn scan_from_gui(path: String) -> String {
    griffon_core::scan(&path)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![scan_from_gui])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
