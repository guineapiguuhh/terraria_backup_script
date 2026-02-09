use std::{
    fs::{copy, read_dir},
    io::{self},
    path::{Path, PathBuf},
};

pub fn copy_dir<P: AsRef<Path>>(from: P, to: P) -> io::Result<()> {
    for dir in read_dir(&from)? {
        let entry = match dir {
            Ok(x) => x,
            Err(_) => continue,
        };
        let path = entry.path();
        if let Ok(t) = entry.file_type()
            && t.is_dir()
        {
            continue;
        }

        let mut from_path = PathBuf::new();
        from_path.push(&from);

        let mut to_path = PathBuf::new();
        to_path.push(&to);

        if let (Some(x), Some(y)) = (path.file_name(), path.extension()) {
            from_path.push(x);
            from_path.set_extension(y);

            to_path.push(x);
            to_path.set_extension(y);
        }

        copy(from_path, to_path)?;
    }
    Ok(())
}
