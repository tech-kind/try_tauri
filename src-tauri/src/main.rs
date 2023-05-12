// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Serialize, Deserialize};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// #[tauri::command]
// fn simple_command() {
//     println!("I was involed from JS!")
// }

// #[tauri::command]
// fn command_with_message(message: String) -> String {
//     format!("hello {}", message)
// }

#[derive(Debug, Serialize, Deserialize)]
struct MyMessage {
    field_str: String,
    field_u32: u32,
}

#[tauri::command]
fn command_with_object(message: MyMessage) -> MyMessage {
    let MyMessage {
        field_str,
        field_u32,
    } = message;

    MyMessage { 
        field_str: format!("hello {}", field_str),
        field_u32: field_u32 + 1,
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            // simple_command,
            // command_with_message
            command_with_object
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
