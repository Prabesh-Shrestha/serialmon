// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use serialmon_lib::*;
fn main() {
        tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            list_serial_ports,
            read_serial_data,
            write_serial_data
        ])
        .run(tauri::generate_context!())
        .expect("Error while running Tauri application");
}
