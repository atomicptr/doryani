use std::{
    fs::{create_dir_all, File},
    io::read_to_string,
};

use directories::ProjectDirs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub disable_default_filters: Option<bool>,
    pub filter: Vec<Filter>,
}

#[derive(Debug, Deserialize)]
pub struct Filter {
    pub name: Option<String>,
    pub url: String,
}

fn defaults() -> Vec<Filter> {
    vec![
        Filter {
            name: Some("NeverSinks Litefilter".to_string()),
            url: "https://raw.githubusercontent.com/NeverSinkDev/NeverSink-PoE2litefilter/refs/heads/main/NeverSinks%20Litefilter.filter".to_string(),
        },
        Filter {
            name: Some("Pechams Loot Filter - Early Game".to_string()),
            url: "https://pastebin.com/raw/8RCBmSYK".to_string(),
        },
        Filter {
            name: Some("Pechams Loot Filter - ENDGAME".to_string()),
            url: "https://pastebin.com/raw/7NPggFSF".to_string(),
        },
    ]
}

pub fn load_config() -> Result<Config, String> {
    let project_dirs = ProjectDirs::from("", "", "doryani").unwrap();
    let config_dir = project_dirs.config_dir().to_path_buf();

    if !config_dir.exists() {
        let _ = create_dir_all(config_dir.clone()).unwrap();
    }

    let config_file = config_dir.join("config.toml");

    if !config_file.exists() {
        return Ok(Config {
            disable_default_filters: None,
            filter: defaults(),
        });
    }

    let f = File::open(config_file).unwrap();
    let data = read_to_string(f).unwrap();

    let mut config: Config = toml::from_str(data.as_str()).unwrap();

    let add_default_filters = !config.disable_default_filters.or(Some(false)).unwrap();

    if add_default_filters {
        let mut default_filters = defaults();
        config.filter.append(&mut default_filters);
    }

    Ok(config)
}
