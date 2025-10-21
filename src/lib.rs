#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    eprintln!("This application is designed for macOS menu bar only");
}
