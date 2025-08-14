use std::fs::{FileType, DirEntry, create_dir, read_dir, copy};
use std::path::{PathBuf, Path};
use std::ffi::OsStr;

#[allow(unused)]
pub fn copy_folder(from: &str, to: &str) {
    for dir in read_dir(from).unwrap() {
        let entry: DirEntry = dir.unwrap();
        let path: PathBuf = entry.path();
        let name: &OsStr = path.file_name().unwrap();
        let _type: FileType = entry.file_type().unwrap();

        if _type.is_dir() {
            create_dir(format!(
                "{}\\{}",
                to,
                name.to_str().unwrap()
            ));

            for sub_dir in read_dir(&path).unwrap() {
                let sub_entry: DirEntry = sub_dir.unwrap();
                let sub_path: PathBuf = sub_entry.path();
                let sub_name: &OsStr = sub_path.file_name().unwrap();
                let sub_type: FileType = sub_entry.file_type().unwrap();

                let parent: &Path = sub_path.parent().unwrap();

                copy_folder(
                    &parent.to_str().unwrap(), 
                    &format!("{}\\{}", to, &name.to_str().unwrap())
                );
            }
            continue;
        }

        copy(
            format!("{}", &path.to_str().unwrap()), 
            format!("{}\\{}", to, &name.to_str().unwrap())
        );
    }
}