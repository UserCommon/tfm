// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod error;
mod commands;

use commands::*;
use error::DirError;
use std::env;




fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_path, get_home, split_dir, ls, open_file, create_dir, create_file, delete])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
