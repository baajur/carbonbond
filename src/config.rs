use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::fs::File;
use serde::{Serialize, Deserialize};
use failure::Fallible;

#[derive(Debug, Serialize, Deserialize)]
pub struct RawConfig {
    pub server: RawServerConfig,
    pub database: RawDatabaseConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawServerConfig {
    pub address: String,
    pub port: u64,
    pub mailgun_key_file: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawDatabaseConfig {
    pub url: String,
}

#[derive(Debug)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug)]
pub struct ServerConfig {
    pub address: String,
    pub port: u64,
    pub mailgun_api_key: Vec<u8>,
}

#[derive(Debug)]
pub struct DatabaseConfig {
    pub url: String,
}

impl From<RawConfig> for Fallible<Config> {
    fn from(orig: RawConfig) -> Fallible<Config> {
        Ok(Config {
            server: Fallible::<ServerConfig>::from(orig.server)?,
            database: Fallible::<DatabaseConfig>::from(orig.database)?,
        })
    }
}

impl From<RawServerConfig> for Fallible<ServerConfig> {
    fn from(orig: RawServerConfig) -> Fallible<ServerConfig> {
        let mailgun_api_key = {
            let mut buf = vec![];
            let mut file = File::open(orig.mailgun_key_file)?;
            file.read_to_end(&mut buf)?;
            buf
        };

        Ok(ServerConfig {
            address: orig.address,
            port: orig.port,
            mailgun_api_key,
        })
    }
}

impl From<RawDatabaseConfig> for Fallible<DatabaseConfig> {
    fn from(orig: RawDatabaseConfig) -> Fallible<DatabaseConfig> {
        Ok(DatabaseConfig { url: orig.url })
    }
}

pub fn load_config<P: AsRef<Path>>(path: P) -> Fallible<Config> {
    // 載入設定檔
    let content = {
        let pathr = path.as_ref();
        let mut file = File::open(pathr)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        content
    };

    let raw_config: RawConfig = toml::from_str(&content)?;
    let config = Fallible::<Config>::from(raw_config)?;
    Ok(config)
}