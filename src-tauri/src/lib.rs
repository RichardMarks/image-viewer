use tauri::{api::dialog::blocking::FileDialogBuilder, AppHandle, Manager};

pub async fn browse_for_image_file(app: AppHandle) -> Result<String, String> {
    let file_path = FileDialogBuilder::new()
        .add_filter("Image Files", &["png"])
        .pick_file();
    if let Some(file_path) = file_path {
        let _ = app.fs_scope().allow_file(file_path.clone());
        return Ok(format!("{:?}", file_path).replace('"', ""));
    }
    Err("".into())
}
