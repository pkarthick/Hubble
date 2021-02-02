extern crate chrono;
extern crate process_folder;

use chrono::offset::Local;
use chrono::DateTime;

use serde::{Deserialize, Serialize};
use std::{fs::{self, Metadata}, thread};
use std::path::Path;

#[derive(Deserialize, Serialize, Debug)]
pub struct Request {
    path: String,
    include_files: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FileInfo {
    path: String,
    size: u64,
    modified_date: String,
    is_duplicate: bool,
    duplicates: Vec<FileInfo>,
}

impl FileInfo {
    pub fn new(path: &Path, metadata: &Metadata) -> FileInfo {
        let datetime: DateTime<Local> = metadata.modified().unwrap().into();
        FileInfo {
            path: String::from(path.to_str().unwrap()),
            size: metadata.len(),
            modified_date: format!("{}", datetime.format("%Y-%m-%d %R")),
            is_duplicate: false,
            duplicates: vec![],
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Folder {
    name: String,
    size: Option<i64>,
    pub path: String,
    pub folders: Vec<Folder>,
    pub files: Vec<FileInfo>,
}

pub async fn get_folder_size(request: Request) -> i64 {
    let path = request.path.clone();
    process_folder::process::get_dir_size(path).await
}

pub async fn get_folder_contents(request: Request) -> Folder {
    let path = request.path.clone();

    let handle = thread::spawn(|| async { process_folder::process::cleanup_old_files(path).await });

    let mut files: Vec<FileInfo> = vec![];
    let mut folders: Vec<Folder> = vec![];

    if let Ok(entries) = fs::read_dir(&request.path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.file_type().is_dir() {
                        let path = Path::file_name(&entry.path()).unwrap().to_string_lossy().to_string();
                        let dir_path = String::from(entry.path().to_str().unwrap());
                        // let dir_size = process::get_dir_size(dir_path.clone()).await;
                        folders.push(Folder {
                            name: path,
                            size: None,
                            path: dir_path,
                            folders: vec![],
                            files: vec![],
                        });
                    } else if request.include_files && metadata.file_type().is_file() {
                        files.push(FileInfo::new(&entry.path(), &metadata));
                    }
                } else {
                    println!("Couldn't get metadata for {:?}", entry.path());
                }
            }
        }
    }

    let _ = handle.join().unwrap();

    let path = request.path;
    let mut name = "".into();

    if let Some(n ) = Path::new(&path).file_name() {
        name =  n.to_string_lossy().to_string()
    }

    Folder {
        name,
        size: None,
        path,
        folders,
        files,
    }
}
