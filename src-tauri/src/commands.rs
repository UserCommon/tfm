use std::io::{Result, Error};
use std::fs;
use std::env;

#[tauri::command]
pub fn get_path(curr: String) -> String {
    curr
}

#[tauri::command]
pub fn get_home() -> String {
    match env::home_dir() {
        Some(dir) => dir.to_str().unwrap().to_string(), // this sucks
        None => panic!("Home directory is not found!")
    }
}

#[tauri::command]
pub fn ls(path: String) -> Vec<String> {
    let iter = fs::read_dir(path).expect("ls functions unable to read dir");
    
    let mut v: Vec<String> = vec![];
    for path  in iter {
        v.push(path.expect("unable to get path in ls function").path().display().to_string());
    }

    v
}

#[tauri::command]
/// This function
/// splits string into slice
/// So it will be comfortable to use in frontend
/// in for loops etc.
pub fn split_dir(dir: &str) -> Vec<String> {
    // TODO! because of / it is probably will not work on windows
    dir[1..].split("/").map(|x| {x.to_string()}).collect()
}