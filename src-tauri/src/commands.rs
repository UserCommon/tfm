use std::io::{Result, Error};
use std::fs::{self, File};
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

#[derive(serde::Serialize)]
pub struct FileOrDir {
    pub path: String,
    pub selected: bool 
}

impl FileOrDir {
    fn new(path: String) -> Self {
        Self {
            path: path,
            selected: false,
        }
    }
}

#[tauri::command]
pub fn ls(path: String) -> Vec<FileOrDir> {
    let mut v: Vec<FileOrDir> = vec![];

    //if path.is_empty() {
    //    return v;
    //}

    let iter = fs::read_dir(path).expect("ls functions unable to read dir");
    
    for path  in iter {
        v.push(FileOrDir::new(path.expect("unable to get path in ls function").path().display().to_string()))
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