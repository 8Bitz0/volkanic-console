use crate::runner;

#[tauri::command]
pub async fn is_valid_url(url: String) -> bool {
    runner::is_valid_url(url)
}
