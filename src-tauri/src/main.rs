use std::env;
use tauri::{AppHandle, Manager};

#[tauri::command]
fn process_input(content: String) {
    println!("{}", content.trim());
    std::process::exit(0);
}

#[tauri::command]
fn show_window(app: AppHandle) {
    let window = app.get_window("main").unwrap();
    if window.is_visible().unwrap() {
        window.hide().unwrap();
        app.exit(0);
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![process_input, show_window])
        .run(tauri::generate_context!())
        .expect("Failed to run Tauri application");
}
