use home;
use serde::Deserialize;
use std::fs;
use toml;

#[derive(Deserialize, Debug)]
pub struct PChatConfig {
    pub pchat_conf: PChatConf,
}
#[derive(Deserialize, Debug)]
pub struct PChatConf {
    pub chat_url: String,
    pub chat_port: u16,
    pub model: String,
}

pub fn get_conf() -> String {
    let mut conf_path = String::new();
    match home::home_dir() {
        Some(path) if !path.as_os_str().is_empty() => {
            conf_path = format!("{}/.config/pchat/config.toml", path.display());
        }
        _ => println!("Unable to find your home dir!"),
    }
    return conf_path;
}

pub fn parse_conf() -> PChatConf {
    let conf_path = get_conf();
    let pchat_conf_contents =
        fs::read_to_string(conf_path).expect("Failed to read config file `config.toml`");
    let pchat_conf: PChatConf = toml::from_str(&pchat_conf_contents).expect("Failed to parse TOML");
    return pchat_conf;
}
