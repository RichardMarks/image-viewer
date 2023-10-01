// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use image_viewer::browse_for_image_file;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![browse_for_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn browse_for_file(app: tauri::AppHandle) -> Result<String, String> {
    browse_for_image_file(app).await
}
