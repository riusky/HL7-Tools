#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod hl7_client;
use hl7_client::hl7_client::{Hl7Client, SendMethod};

// Prevents additional console window on Windows in release, DO NOT REMOVE!!

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn send_hl7_message(message: &str, server_address: &str, port: &str, method: &str) -> Result<String,String> {

    let result =     Hl7Client::new(
            message.to_owned(),
            server_address.to_owned(),
            port.parse().unwrap(),
            match method {
                "TCP" => SendMethod::Tcp,
                "HTTP" => SendMethod::Http,
                _ => SendMethod::Tcp,
            },
        )
        .send_message()
        .await;
  

    match result {
        Ok(response) => Ok(response),
        Err(e) => Err(e.to_string()),
    }
    // "".to_owned()
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet,send_hl7_message])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
