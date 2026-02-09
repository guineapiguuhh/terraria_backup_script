use std::{env::home_dir, path::PathBuf};

pub fn get_data_path() -> Option<PathBuf> {
    let home_path = home_dir()?;

    let mut data_path = home_path;
    data_path.push("Documents");
    data_path.push("My Games");
    data_path.push("Terraria");

    Some(data_path)
}
