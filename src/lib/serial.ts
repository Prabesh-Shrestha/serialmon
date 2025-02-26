import { invoke } from "@tauri-apps/api/core";

export type SerialPort = string;
export type BaudRate = "9600" | "115200" | "57600" | "38400" | "19200" | "4800";

// Fetch list of available serial ports
export async function getSerialPorts(): Promise<SerialPort[]> {
  return await invoke("list_serial_ports");
}

// Send data to the serial port
export async function sendSerialData(port: string, baudRate: BaudRate, message: string): Promise<void> {
  await invoke("write_serial_data", { port_name: port, baud_rate: parseInt(baudRate), message });
}

// Read data from the serial port
export async function readSerialData(port: string, baudRate: BaudRate): Promise<string> {
  return await invoke("read_serial_data", { port_name: port, baud_rate: parseInt(baudRate) });
}

