#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;

// פקודה שה-JS יקרא לה כשהוא מוכן
#[tauri::command]
fn show_main_window(window: tauri::Window) {
  window.show().unwrap();
  window.set_focus().unwrap();
}

fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_sql::Builder::default().build())
    .plugin(tauri_plugin_window_state::Builder::default().build())
    .invoke_handler(tauri::generate_handler![show_main_window]) // רישום הפקודה
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
