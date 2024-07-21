use std::collections::HashMap;
use std::path::PathBuf;

use dirs;
use once_cell::sync::Lazy;
use crate::config::{Config, Packages};

const EPHER_TMP_PATH: &str = "/tmp/epher";
const EPHER_VAR_TMP_PATH: &str = "/var/tmp/epher";
/*
static EPHER_HOME_PATH: Lazy<PathBuf> = Lazy::new(|| {
    let m = home_dir().unwrap().clone().join(".epher");
    m
});
static EPHER_APP_PATH: Lazy<PathBuf> = Lazy::new(|| {
    EPHER_HOME_PATH.join("app")
});*/
/*
static EPHER_APP_PATH: &str = concat!(EPHER_HOME_PATH.,"a");*/

pub static ENV_INFO :Lazy<EnvInfo>= Lazy::new(||{
    //TODO: Load Config from File
    let mut map = HashMap::new();
    map.insert(String::from("git"), Packages {
        url: String::from("https://aaaa")
    });
    map.insert(String::from("gh"), Packages {
        url: String::from("https://gh")
    });
    let config = Config {
        ip: "127.0.0.1".to_string(),
        packages: map,
    };
    EnvInfo::new(config)
});

pub struct EnvInfo {
    pub tmp_path: PathBuf,
    pub var_tmp_path: PathBuf,
    pub home_path: PathBuf,
    pub config: Config,
}

impl EnvInfo {
    fn new(config: Config) -> Self {
        Self {
            tmp_path: PathBuf::from(EPHER_TMP_PATH),
            var_tmp_path: PathBuf::from(EPHER_VAR_TMP_PATH),
            home_path: dirs::home_dir().unwrap().join(".epher"),
            config,
        }
    }
    fn load_envvar(){
        todo!()
    }
    
    fn update(&mut self) {
        self.home_path = dirs::home_dir().unwrap().join(".epher")
    }
}
