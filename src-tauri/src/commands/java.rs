use crate::services::java_detector;

#[tauri::command]
pub fn detect_java() -> Vec<java_detector::JavaInfo> {
    java_detector::detect_java_installations()
}

#[tauri::command]
pub fn validate_java_path(path: String) -> Result<java_detector::JavaInfo, String> {
    java_detector::validate_java(path.as_str())
}
