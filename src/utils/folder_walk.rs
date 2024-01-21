// #![allow(unused)] // For beginning only.

use std::fs;

use crate::prelude::*;

// pub struct FolderItem {
//     pub path: String,
//     pub is_dir: bool,
// }

// impl FolderItem {
//     pub fn new(path: String, is_dir: bool) -> Self {
//         Self { path, is_dir }
//     }
// }

// pub fn folder_walk(path: &str) -> Result<Vec<FolderItem>> {
//     let mut paths = vec![];

//     for entry in fs::read_dir(path)? {
//         let entry = entry?;
//         let path = entry.path();
//         let path = path.to_str().unwrap().to_string();
//         let is_dir = entry.file_type()?.is_dir();

//         paths.push(FolderItem::new(path, is_dir));
//     }

//     Ok(paths)
// }

// pub fn get_files(path: &str) -> Result<Vec<String>> {
//     // check if path is a directory
//     if !fs::metadata(path)?.is_dir() {
//         return Err(Error::Generic(format!("{} is not a directory", path)));
//     }

//     let mut paths = vec![];

//     for entry in fs::read_dir(path)? {
//         let entry = entry?;
//         let path = entry.path();
//         let path = path.to_str().unwrap().to_string();
//         let is_dir = entry.file_type()?.is_dir();

//         if !is_dir {
//             paths.push(path);
//         }
//     }

//     Ok(paths)
// }

pub fn get_pdf_files(path: &str) -> Result<Vec<String>> {
    let mut paths = vec![];

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let path = path.to_str().unwrap().to_string().to_lowercase();
        let is_dir = entry.file_type()?.is_dir();

        if !is_dir && path.ends_with(".pdf") {
            paths.push(path);
        }
    }

    Ok(paths)
}
