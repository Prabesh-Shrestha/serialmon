use serialport;
use tauri::command;

use std::io::prelude::*;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
#[command]
pub fn list_serial_ports() -> Vec<String> {
    serialport::available_ports()
        .unwrap()
        .iter()
        .map(|p| p.port_name.clone())
        .collect()
}

#[command]
pub fn read_serial_data(port_name: String, baud_rate: u32) -> String {
    let port = serialport::new(port_name, baud_rate)
        .timeout(std::time::Duration::from_millis(1000))
        .open()
        .expect("Failed to open port");

    let reader = Arc::new(Mutex::new(BufReader::new(port)));

    let mut buffer = String::new();
    {
        let mut reader = reader.lock().unwrap();
        reader.read_line(&mut buffer).expect("Failed to read line");
    }
    buffer
}

#[command]
pub fn write_serial_data(port_name: String, baud_rate: u32, message: String) {
    let mut port = serialport::new(port_name, baud_rate)
        .timeout(std::time::Duration::from_millis(1000))
        .open()
        .expect("Failed to open port");

    port.write_all(message.as_bytes()).expect("Failed to write");
}
