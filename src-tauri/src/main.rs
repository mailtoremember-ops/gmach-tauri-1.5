#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::Manager;

fn main() {
  tauri::Builder::default()
    // רישום התוספים: בסיס נתונים ושמירת מצב חלון
    .plugin(tauri_plugin_sql::Builder::default().build())
    .plugin(tauri_plugin_window_state::Builder::default().build())
    
    // מנגנון להצגת החלון בצורה חלקה רק אחרי שהטעינה הסתיימה (בלי לגעת ב-HTML)
    .on_page_load(|window, _payload| {
      let w = window.clone();
      std::thread::spawn(move || {
        // השהייה קצרה שנותנת ל-Windows זמן למקם את החלון במיקומו הישן לפני החשיפה
        std::thread::sleep(std::time::Duration::from_millis(150));
        w.show().unwrap();
        w.set_focus().unwrap();
      });
    })
    
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
