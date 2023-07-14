use std::io::{Result, Error};
use std::fs::{self, File};
use std::env;
use std::process::Command;
use std::path::PathBuf;

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


#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum FileOrDirType {
    Directory,
    Image,
    File, // Image is file FIXME:
    Unknown
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct FileOrDir {
    pub file_or_dir_type: FileOrDirType,
    pub path: String,
    pub selected: bool,
    pub on_copy: bool,
    pub on_cut: bool,
    pub is_dot: bool,
}

impl FileOrDir {
    fn new(path: String, file_or_dir_type: FileOrDirType) -> Self {
        let dotted: bool = path.split("/").last().unwrap().chars().next().unwrap() == '.';
        // ^ dat shit will cause something horrible i guess FIXME:
        Self {
            file_or_dir_type: file_or_dir_type,
            path: path,
            selected: false,
            on_copy: false,
            on_cut: false,
            is_dot: dotted
        }
    }
}

pub fn file_or_dir_type(path: &PathBuf) -> FileOrDirType {
    if path.is_dir() {
        return FileOrDirType::Directory;
    } else if path.is_file() {
        return FileOrDirType::File;
    }
    FileOrDirType::Unknown
}

#[tauri::command]
pub fn ls(path: String) -> Vec<FileOrDir> {
    let mut v: Vec<FileOrDir> = vec![];

    //if path.is_empty() {
    //    return v;
    //}

    let iter = fs::read_dir(path).expect("ls functions unable to read dir");
    
    for path in iter {
        let path: PathBuf = path.expect("unable to get path in ls").path();
        v.push(
            FileOrDir::new(
                path.display().to_string(),
                file_or_dir_type(&path)
            )
        )
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


#[tauri::command]
#[cfg(target_os = "linux")]
pub fn open_file(path: String) {
    Command::new("xdg-open")
        .arg(path)
        .spawn()
        .unwrap();
}


#[tauri::command]
#[cfg(target_os = "windows")]
pub fn open_file(path: String) {
    // TODO: Test
    Command::new("start")
        .arg(path.chars().map(|x| {prettify(x)}).collect())
        .spawn()
        .unwrap();

    fn prettify(x: char) -> char {
        if x == '/' {return '\\';}
        x
    }
}


#[tauri::command]
#[cfg(target_os="macos")]
pub fn open_file(path: String) {
    Command::new("open")
        .arg(path)
        .spawn()
        .unwrap();
}

#[tauri::command]
pub fn create_dir(path: String, name: String) {
    // Может не надо передавать name?
    fs::create_dir(format!("{}/{}",path, name)).expect("Failed to create directory in create_dir");
}

#[tauri::command]
pub fn create_file(path: String, name: String) {
    fs::File::create(format!("{}/{}", path, name)).expect("Failed to create file in create_file");
}

#[tauri::command]
pub fn delete(file: FileOrDir) {
    match file.file_or_dir_type {
        FileOrDirType::Directory => {
            fs::remove_dir_all(file.path).expect("Can't remove! in delete");
        }
        _ => {
            fs::remove_file(file.path).expect("Can't remove! in delete");
        }
    };
}