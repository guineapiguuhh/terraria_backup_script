#[path = "utils.rs"] mod utils;

use utils::copy_folder;
use std::fs::create_dir;

use chrono::{Local, NaiveDate};

const SPLASH_TEXT: &'static str = "Terraria Backup Script";

pub fn start(path: String, my_game_path: String) {
    let folder: String = create_folders(&path);

    println!("[{}]", SPLASH_TEXT);
    println!("> [Ctrl + C] = Stop Script\n");

    println!("> 1. Vanilla Players Backup");
    copy_folder(
        &format!("{}\\Players", my_game_path), 
        &format!("{}\\Players", folder), 
    );
    
    println!("> 2. Vanilla Worlds Backup");
    copy_folder(
        &format!("{}\\Worlds", my_game_path), 
        &format!("{}\\Worlds", folder), 
    );

    println!("> 3. Modded Players Backup");
    copy_folder(
        &format!("{}\\tModLoader\\Players", my_game_path), 
        &format!("{}\\tModLoader\\Players", folder), 
    );
    
    println!("> 4. Modded Worlds Backup");
    copy_folder(
        &format!("{}\\tModLoader\\Worlds", my_game_path), 
        &format!("{}\\tModLoader\\Worlds", folder), 
    );
}

#[allow(unused)]
pub fn create_folders(path: &String) -> String {
    let now: NaiveDate = Local::now().date_naive();
    let name: String = format!(
        "Terraria {}",
        now.format("%d-%m-%Y")
    );

    let folder_path: &String = &format!(
        "{}\\{}", 
        path,
        name
    );

    create_dir(folder_path);

    create_dir(format!("{}\\Players", folder_path));
    create_dir(format!("{}\\Worlds", folder_path));

    create_dir(format!("{}\\tModLoader", folder_path));
    create_dir(format!("{}\\tModLoader\\Players", folder_path));
    create_dir(format!("{}\\tModLoader\\Worlds", folder_path));

    return folder_path.clone();
}