#[path = "backup.rs"] mod backup;
use backup::start;

use std::env::home_dir;
use std::path::PathBuf;

fn get_home_path() -> String {
    let home_dir: PathBuf = home_dir().unwrap();

    return String::from(home_dir.to_str().unwrap());
}

fn main() {
    let path: String = get_home_path() + "\\Desktop\\Backup\\Terraria [Backup]";
    let my_game_path: String = get_home_path() + "\\Documents\\My Games\\Terraria";

    start(path, my_game_path);
}