#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
pub mod common;
pub mod plane;
pub mod beno;

use commands::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
