#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod beano;
mod commands;
pub mod common;
pub mod plane;
pub mod simulation;

use commands::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![load_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
