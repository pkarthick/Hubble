use sqlite_data::data::{CreateData, DeleteData, QueryAsBuilder, QueryBuilder, ReadData, UpdateData};

use async_std::path::Path;

use chrono::prelude::*;

use std::{convert::TryInto, time::UNIX_EPOCH};

pub struct Drives {
    drives: Vec<Drive>,
}

impl Drives {
    pub fn new() -> Self {
        //call database and return the drives

        Self {
            drives: vec![
                Drive {
                    id: 1,
                    path: "/media/karthick/Data",
                },
                Drive {
                    id: 2,
                    path: "/media/karthick/Elements",
                },
            ],
        }
    }

    pub fn get(&self, path: &str) -> Option<&Drive> {
        self.drives.iter().find(|d| path.starts_with(&d.path))
    }

}

#[derive(Clone)]
pub struct Drive {
    pub id: i64,
    pub path: &'static str,
}

impl Drive {
    pub fn get_relative_path<'a>(&self, path: &'a str) -> &'a str {
        path[self.path.len()..].trim_start_matches('/')
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct FileInfo {
    pub driveid: i64,
    pub name: String,
    pub size: i64,
    pub folderpath: String,
    pub hash: Option<String>,
    pub modified: String,
}

impl FileInfo {
    pub fn new(path: &Path, drive: &Drive) -> Self {
        let metadata = std::fs::metadata(path).unwrap();

        let parent = path
            .parent()
            .unwrap()
            .to_string_lossy()
            .into_owned()
            .strip_prefix(&drive.path)
            .unwrap()
            .trim_start_matches('/')
            .into();

        let mut secs = 0;

        if let Ok(time) = metadata.modified() {
            if let Ok(time) = time.duration_since(UNIX_EPOCH) {
                secs = time.as_secs() as i64;
            } else {
                println!("Modified date issue for {:?}", path);
            }
        } else {
            println!("Metadata issue for {:?}", path);
        }

        FileInfo {
            driveid: drive.id,
            name: path.file_name().unwrap().to_string_lossy().into_owned(),
            size: metadata.len().try_into().unwrap(),
            folderpath: parent,
            hash: None,
            modified: Local.timestamp(secs, 0).to_rfc3339(),
        }
    }

    pub fn is_modified(&self, path: &Path) -> Option<String> {
        let metadata = std::fs::metadata(path).unwrap();

        let newtime = metadata
            .modified()
            .unwrap()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let oldtime = DateTime::parse_from_rfc3339(self.modified.as_str())
            .unwrap()
            .timestamp();

        let modified = newtime > oldtime;

        if modified {
            Some(Local.timestamp(newtime, 0).to_rfc3339())
        } else {
            None
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct FolderInfo {
    pub id: Option<i64>,
    pub path: String,
    pub size: Option<i64>,
    pub parentid: Option<i64>,
    pub modified: String,
    pub files_size: Option<i64>,
}

impl FolderInfo {
    pub fn new(path: &str) -> Self {
        let metadata = std::fs::metadata(path).unwrap();
        let secs = metadata
            .modified()
            .unwrap()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .try_into()
            .unwrap();
        Self {
            id: None,
            path: path.into(),
            size: None,
            modified: Local.timestamp(secs, 0).to_rfc3339(),
            parentid: None,
            files_size: None,
        }
    }

    pub fn check_modified(&self) -> bool {
        let res = std::fs::metadata(&self.path);

        if let Ok(metadata) = res {
            let newtime: i64 = metadata
                .modified()
                .unwrap()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs()
                .try_into()
                .unwrap();

            let oldtime = DateTime::parse_from_rfc3339(self.modified.as_str())
                .unwrap()
                .timestamp();

            newtime > oldtime
        } else {
            false
        }
    }
}

impl CreateData<FileInfo> for FileInfo {
    fn create(&self) -> QueryBuilder {
        QueryBuilder::new(
            "INSERT INTO Files (driveid, name, size, folderpath, hash, modified) values (?, ?, ?, ?, ?, ?)",
        )
        .bind(self.driveid)
        .bind(&self.name)
        .bind(self.size)
        .bind(&self.folderpath)
        .bind(&self.hash)
        .bind(&self.modified)
    }
}

impl UpdateData<FileInfo> for FileInfo {
    fn update(&self) -> QueryBuilder {
        QueryBuilder::new(
            "UPDATE Files set size=?, modified=? where name=? and folderpath=? and driveid=?",
        )
        .bind(self.size)
        .bind(&self.modified)
        .bind(&self.name)
        .bind(&self.folderpath)
        .bind(self.driveid)
    }
}

impl DeleteData<FileInfo> for FileInfo {
    fn delete(&self) -> QueryBuilder {
        QueryBuilder::new("DELETE FROM Files where name=? and folderpath=? and driveid=?")
            .bind(&self.name)
            .bind(&self.folderpath)
            .bind(self.driveid)
    }
}

impl ReadData<FileInfo> for FileInfo {
    fn read(&self) -> QueryAsBuilder<FileInfo> {
        QueryAsBuilder::new(r#"SELECT * FROM Files where name = ? and folderpath=? and driveid=?"#)
            .bind(&self.name)
            .bind(&self.folderpath)
            .bind(self.driveid)
    }
}
