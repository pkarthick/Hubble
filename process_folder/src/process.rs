use sqlite_data::data::{DataContext, QueryAsBuilder};

use crate::model::{Drive, Drives, FileInfo, FolderInfo};

use async_std::{fs, path::Path, prelude::*};
use std::{collections::VecDeque, thread};

pub async fn process(path: String) {
    let mut tasks = vec![];
    let mut folders = VecDeque::new();
    folders.push_front(path);

    loop {
        let cpus = 6;
        let folderopt = folders.pop_front();

        if folderopt.is_none() {
            if tasks.is_empty() {
                break;
            }
        } else if tasks.len() < cpus {
            let path = folderopt.clone().unwrap();
            tasks.push(thread::spawn(|| process_folder(path)));
            continue;
        }

        for task in tasks.drain(..) {
            let mut sfs = task.join().unwrap().await;
            for sf in sfs.drain(..) {
                folders.push_front(sf);
            }
        }
    }
}

async fn get_all_files(context: &mut DataContext, drive: &Drive, path: String) -> Vec<FileInfo> {
    let folderpath = drive.get_relative_path(&path);
    let builder = QueryAsBuilder::<FileInfo>::new(
        r#"SELECT * FROM Files where instr(folderpath, ?) = 1 and driveid = ?"#,
    )
    .bind(&folderpath)
    .bind(drive.id);

    context.query(builder).await
}

pub async fn cleanup_old_files(path: String) {
    let p = std::env::current_dir().unwrap().join("data.db");
    let url = format!("sqlite:://{}", p.to_str().unwrap());
    let mut context = DataContext::new(&url).await;
    let drives = Drives::new();
    let drive = drives.get(&path).unwrap();

    let existing_files = get_all_files(&mut context, drive, path).await;

    for file in existing_files.into_iter() {
        let path = Path::new(drive.path)
            .join(file.folderpath.clone())
            .join(file.name.clone());
        if !path.exists().await {
            println!("Deleting! -> {:?}", path);
            context.delete(file).await;
        }
    }
}

pub async fn get_dir_size(path: String) -> i64 {
    let mut size = 0_i64;

    let p = std::env::current_dir().unwrap().join("data.db");
    let url = format!("sqlite:://{}", p.to_str().unwrap());

    let mut context = DataContext::new(&url).await;
    let drives = Drives::new();
    if let Some(drive) = drives.get(&path) {

    let existing_files = get_all_files(&mut context, drive, path).await;

    for file in existing_files.into_iter() {
        let path = Path::new(drive.path)
            .join(file.folderpath.clone())
            .join(file.name.clone());
        if path.exists().await {
            size += file.size;
        }
    }
}

    size
}

async fn process_folder(path: String) -> Vec<String> {
    let p = std::env::current_dir().unwrap().join("data.db");
    let url = format!("sqlite:://{}", p.to_str().unwrap());

    let mut context = DataContext::new(&url).await;
    let drives = Drives::new();
    let drive = drives.get(&path).unwrap();

    let folder = FolderInfo::new(&path);

    let mut subfolders = vec![];
    let mut entries = fs::read_dir(path).await.unwrap();

    while let Some(res) = entries.next().await {
        let entry = res.unwrap();
        let metadata = entry.metadata().await.unwrap();

        if metadata.file_type().is_dir() {
            let fp = String::from(entry.path().to_str().unwrap());
            subfolders.push(fp);
        } else {
            if folder.check_modified() {
                let mut file = FileInfo::new(&entry.path(), drive);

                if let Some(modified) = file.is_modified(&entry.path()) {
                    file.modified = modified;
                }

                if let Some(_) = context.find(&file).await {
                    context.update(&file).await;
                } else {
                    context.create(&file).await;
                }
            }
        }
    }

    subfolders
}
