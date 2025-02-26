// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
pub mod serial;
pub use serial::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
}
