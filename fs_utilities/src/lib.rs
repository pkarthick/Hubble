extern crate chrono;
use chrono::offset::Local;
use chrono::DateTime;

use serde::{Deserialize, Serialize};
use std::fs::{self, Metadata};
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
    pub path: String,
    pub folders: Vec<Folder>,
    pub files: Vec<FileInfo>,
}

// pub async fn folder(request: web::Json<Request>) -> web::Json<Folder> {
//     println!("=========={:?}=========", request);
//     web::Json(get_folder_contents(request.0))
// }

pub fn get_folder_contents(request: Request) -> Folder {
    let path = request.path;
    let mut files: Vec<FileInfo> = vec![];
    let mut folders: Vec<Folder> = vec![];

    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.file_type().is_dir() {
                        folders.push(Folder {
                            path: String::from(entry.path().to_str().unwrap()),
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

    Folder {
        path,
        folders,
        files,
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
