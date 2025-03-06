use std::sync::{Arc, Mutex};
use tauri::{Emitter, State};

#[warn(unused_imports)]
use serialport::{self, SerialPort};

// Add proper state management struct
#[derive(Default)]
struct SerialState {
    port: Arc<Mutex<Option<Box<dyn serialport::SerialPort + Send>>>>,
}

#[tauri::command]
async fn list_ports() -> Result<Vec<String>, String> {
    serialport::available_ports()
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(|port| Ok(port.port_name))
        .collect()
}

#[tauri::command]
async fn connect(
    port_name: String,
    baud_rate: u32,
    state: State<'_, SerialState>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let port = serialport::new(&port_name, baud_rate)
        .timeout(std::time::Duration::from_millis(10))
        .open()
        .map_err(|e| e.to_string())?;

    // Store the port in the state
    *state.port.lock().unwrap() = Some(port);

    let app_handle_clone = app_handle.clone();
    let port_arc = Arc::clone(&state.port);

    std::thread::spawn(move || {
        let mut buf = [0; 1024];
        loop {
            let mut guard = match port_arc.lock() {
                Ok(g) => g,
                Err(poisoned) => poisoned.into_inner(),
            };

            if let Some(port) = guard.as_deref_mut() {
                match port.read(&mut buf) {
                    Ok(t) if t > 0 => {
                        let data = String::from_utf8_lossy(&buf[..t]).to_string();
                        let _ = app_handle_clone.emit("serial-data", data.clone());
                        port.flush().unwrap();
                        buf.iter_mut().for_each(|x| *x = 0);
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::TimedOut => (),
                    Err(e) => {
                        eprintln!("Read error: {}", e);
                        break;
                    }
                    _ => (),
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    });

    Ok(())
}

#[tauri::command]
async fn send_message(
    message: String,
    state: State<'_, SerialState>,
) -> Result<(), String> {
    if let Some(port) = &mut *state.port.lock().unwrap() {
        port.write_all(message.as_bytes())
            .map_err(|e| e.to_string())?;
        port.flush().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Properly manage the state
        .manage(SerialState::default())
        .invoke_handler(tauri::generate_handler![list_ports, connect, send_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
