use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

const APP_NAME: &str = "FUNFRIEND";
const CONFIG_FILE_NAME: &str = "cfg.json";

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Friends {
    FUNFRIEND,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ConfigType {
    pub friend_size: u32,
    pub volume: f32,
    pub friend_type: Friends,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub config: ConfigType,
    pub path: PathBuf,
}

impl Default for ConfigType {
    fn default() -> Self {
        Self {
            friend_size: 75,
            volume: 0.2,
            friend_type: Friends::FUNFRIEND,
        }
    }
}

impl Config {
    pub fn retrieve_config_path() -> Option<PathBuf> {
        //@NOTE currently assuming everyone is in linux,
        //@TODO implement other platforms
        let home = env::var("HOME").expect("unable to find var");
        let path = Path::new(&home).join(".config").join(APP_NAME);
        Some(path)
    }

    pub fn init() -> Result<Self, io::Error> {
        let path = Self::retrieve_config_path().expect("unable to retrieve config path");
        if let Err(e) = fs::create_dir(&path) {
            if e.kind() != io::ErrorKind::AlreadyExists {
                return Err(e);
            }
        }

        let file = Path::new(&path).join(CONFIG_FILE_NAME);
        if !file.exists() {
            let config = ConfigType::default();
            let json = serde_json::to_string(&config).expect("unable to serialize conf");
            fs::write(&file, json.as_str())?;
            return Ok(Self { config, path: file });
        } else {
            let contents = fs::read_to_string(&file).expect("unable to read file");
            let config = serde_json::from_str::<ConfigType>(contents.as_str())
                .expect("unable to deserialize configtype");
            return Ok(Self { config, path: file });
        }
    }
}
