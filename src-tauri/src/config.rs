use std::fs;
use std::{fs::File, io::Write};

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum SaveUnitType {
    File,
    Folder,
}
#[derive(Debug, Serialize, Deserialize)]
struct SaveUnit {
    unit_type: SaveUnitType,
    path: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Game {
    save_paths: Vec<SaveUnit>,
    game_path: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
struct Settings {
    prompt_when_not_described: bool,
    extra_backup_when_apply: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    version: String,
    backup_path: String,
    games: Vec<Game>,
    settings: Settings,
}

fn default_config() -> Config {
    Config {
        version: String::from("0.4.0"),
        backup_path: String::from("./save_data"),
        games: Vec::new(),
        settings: Settings {
            prompt_when_not_described: false,
            extra_backup_when_apply: true,
        },
    }
}

fn init_config() -> Result<()> {
    println!("初始化配置文件");
    let json = serde_json::to_string_pretty(&default_config())?;
    let mut file = File::create("./GameSaveManager.config.json")?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn get_config() -> Result<Config> {
    let file = File::open("./GameSaveManager.config.json")?;
    Ok(serde_json::from_reader(file)?)
}

pub fn set_config(config: Config) -> Result<()> {
    let mut file = File::create("./GameSaveManager.config.json")?;
    file.write_all(serde_json::to_string_pretty(&config)?.as_bytes())?;
    Ok(())
}

#[tauri::command]
async fn config_check()->Result<String>{
    if !fs::metadata("./GameSaveManager.config.json")?.is_file() {
        init_config()?;
    }
    let config = get_config()?;
    if config.version != default_config().version{
        //TODO:需要完成旧版本到新版本的迁移
        todo!();
    }
    Ok(serde_json::to_string(&config)?)
}

#[cfg(test)]
mod test {
    use super::default_config;
    use anyhow::Result;

    #[test]
    fn serialize_default_config() -> Result<()> {
        let config = default_config();
        let json = serde_json::to_string(&config)?;
        println!("序列化得到:\n{}", json);
        Ok(())
    }
}