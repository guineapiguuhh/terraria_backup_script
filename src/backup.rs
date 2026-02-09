use std::{
    fs::{create_dir, exists},
    io,
};

use chrono::Local;

use crate::{file_utils::copy_dir, terraria};

#[derive(Debug)]
pub enum Step {
    Players,
    Worlds,
    TModLoaderPlayers,
    TModLoaderWorlds,
}

impl Step {
    pub fn to_dir(&self) -> String {
        match self {
            Self::Players => "Players",
            Self::Worlds => "Worlds",
            Self::TModLoaderPlayers => "tModLoaderPlayers",
            Self::TModLoaderWorlds => "tModLoaderWorlds",
        }
        .to_owned()
    }

    pub fn all_steps() -> [Step; 4] {
        [
            Step::Players,
            Step::Worlds,
            Step::TModLoaderPlayers,
            Step::TModLoaderWorlds,
        ]
    }
}

pub fn create_backup_folders(path: &str) -> String {
    let now = Local::now().date_naive();

    let folder_path = format!("{}\\{}", path, now.format("Terraria %d-%m-%Y"));
    let _ = create_dir(&folder_path);
    let _ = create_dir(format!("{}\\Players", &folder_path));
    let _ = create_dir(format!("{}\\Worlds", &folder_path));

    folder_path
}

pub fn copy_to_backup_folder(
    path: &str,
    available_steps: &[Step],
    on_step: impl Fn(&Step),
) -> Result<(), io::Error> {
    let data_path = terraria::get_data_path()
        .ok_or(io::Error::from(io::ErrorKind::NotFound))?
        .to_str()
        .ok_or(io::Error::from(io::ErrorKind::InvalidData))?
        .to_string();

    for step in available_steps {
        let dir = step.to_dir();

        let from = format!("{}\\{}", data_path, dir);
        if !exists(&from)? {
            continue;
        }

        copy_dir(from, format!("{}\\{}", path, dir))?;
        on_step(step);
    }
    Ok(())
}
