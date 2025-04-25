use std::{env, fs, io, path::{Path, PathBuf}};
const APP_NAME: &str = "FUNFRIEND";
const CONFIG_FILE_NAME: &str = "cfg.ini";

pub enum Friends {
    FUNFRIEND
}
pub struct ConfigType {
    friend_size: i64,
    volume: f32,
    friend_type: Friends
}
pub struct Config {
    config: ConfigType,
    path: String,
    initialized: bool
}

impl Config {
    pub fn new() -> Self {
        todo!()
    }
    pub fn retrieve_config_path() -> Option<PathBuf> {
        //@NOTE currently assuming everyone is in linux,
        //@TODO implement other platforms
        let home = env::var("HOME").expect("unable to find HOME var");
        let path = Path::new(&home).join(".config").join(APP_NAME);
        Some(path)
    }

    pub fn init(&mut self) -> Result<(), io::Error> {
        let path = Self::retrieve_config_path().expect("unable to retrieve config path");
        fs::create_dir(path)?;
        self.initialized = true;
        Ok(())
    }
    
    pub fn config(&mut self) -> Result<ConfigType, &'static str> {
        if !self.initialized {
            return Err("config read before init")
        }
        todo!()
    }
}