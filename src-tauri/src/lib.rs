use once_cell::sync::Lazy;
use std::env;
use tauri::{AppHandle, Listener, Manager};
use tauri_nspanel::ManagerExt;
use window::WebviewWindowExt;

static APP_CONFIG: Lazy<config::Config> =
    Lazy::new(|| config::Config::new().expect("Failed to load configuration"));

mod config;
mod window;

pub const PANEL_LABEL: &str = "main";

#[tauri::command]
fn process_input(content: &str) {
    if content.trim().is_empty() {
        std::process::exit(0);
    }
    println!("{}", content.trim());
    std::process::exit(0);
}

#[tauri::command]
fn show_panel(app: AppHandle) {
    let window = app.get_webview_window(PANEL_LABEL).unwrap();
    let panel = app.get_webview_panel(PANEL_LABEL).unwrap();
    if panel.is_visible() {
        panel.order_out(None);
        app.exit(0);
    } else {
        window.center_at_cursor_monitor().unwrap();
        panel.show();
    }
}

#[tauri::command]
fn get_theme_css() -> Result<String, String> {
    APP_CONFIG.load_css()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            process_input,
            show_panel,
            get_theme_css
        ])
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_nspanel::init())
        .setup(move |app| {
            // Set activation poicy to Accessory to prevent the app icon from showing on the dock
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let handle = app.app_handle();

            let window = handle
                .get_webview_window(PANEL_LABEL)
                .expect("failed to get window");

            let panel = window.to_input_panel()?;
            handle.listen(format!("{}_panel_did_resign_key", PANEL_LABEL), move |_| {
                // Hide the panel when it's no longer the key window
                // This ensures the panel doesn't remain visible when it's not actively being used
                panel.order_out(None);
            });

            show_panel(handle.clone());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
