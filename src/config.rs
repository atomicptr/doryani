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
        // NeverSink
        Filter {
            name: Some("NeverSinks Filter - 0 Soft".to_string()),
            url: "https://raw.githubusercontent.com/NeverSinkDev/NeverSink-Filter-for-PoE2/refs/heads/main/NeverSink's%20filter%202%20-%200-SOFT.filter".to_string(),
        },
        Filter {
            name: Some("NeverSinks Filter - 1 Regular".to_string()),
            url: "https://raw.githubusercontent.com/NeverSinkDev/NeverSink-Filter-for-PoE2/refs/heads/main/NeverSink's%20filter%202%20-%201-REGULAR.filter".to_string(),
        },
        Filter {
            name: Some("NeverSinks Filter - 2 Semi Strict".to_string()),
            url: "https://raw.githubusercontent.com/NeverSinkDev/NeverSink-Filter-for-PoE2/refs/heads/main/NeverSink's%20filter%202%20-%202-SEMI-STRICT.filter".to_string(),
        },
        Filter {
            name: Some("NeverSinks Filter - 3 Strict".to_string()),
            url: "https://raw.githubusercontent.com/NeverSinkDev/NeverSink-Filter-for-PoE2/refs/heads/main/NeverSink's%20filter%202%20-%203-STRICT.filter".to_string(),
        },
        Filter {
            name: Some("NeverSinks Filter - 4 Very Strict".to_string()),
            url: "https://raw.githubusercontent.com/NeverSinkDev/NeverSink-Filter-for-PoE2/refs/heads/main/NeverSink's%20filter%202%20-%204-VERY-STRICT.filter".to_string(),
        },
        Filter {
            name: Some("NeverSinks Filter - 5 Uber Strict".to_string()),
            url: "https://raw.githubusercontent.com/NeverSinkDev/NeverSink-Filter-for-PoE2/refs/heads/main/NeverSink's%20filter%202%20-%205-UBER-STRICT.filter".to_string(),
        },
        Filter {
            name: Some("NeverSinks Filter - 6 Uber Plus Strict".to_string()),
            url: "https://github.com/NeverSinkDev/NeverSink-Filter-for-PoE2/blob/main/NeverSink's%20filter%202%20-%206-UBER-PLUS-STRICT.filter".to_string(),
        },
        // NeverSink Dark Mode
        Filter {
            name: Some("NeverSinks Filter Dark - 0 Soft".to_string()),
            url: "https://raw.githubusercontent.com/NeverSinkDev/NeverSink-Filter-for-PoE2/refs/heads/main/(STYLE)%20DARKMODE/NeverSink's%20filter%202%20-%200-SOFT%20(darkmode)%20.filter".to_string(),
        },
        Filter {
            name: Some("NeverSinks Filter Dark - 1 Regular".to_string()),
            url: "https://raw.githubusercontent.com/NeverSinkDev/NeverSink-Filter-for-PoE2/refs/heads/main/(STYLE)%20DARKMODE/NeverSink's%20filter%202%20-%201-REGULAR%20(darkmode)%20.filter".to_string(),
        },
        Filter {
            name: Some("NeverSinks Filter Dark - 2 Semi Strict".to_string()),
            url: "https://raw.githubusercontent.com/NeverSinkDev/NeverSink-Filter-for-PoE2/refs/heads/main/(STYLE)%20DARKMODE/NeverSink's%20filter%202%20-%202-SEMI-STRICT%20(darkmode)%20.filter".to_string(),
        },
        Filter {
            name: Some("NeverSinks Filter Dark - 3 Strict".to_string()),
            url: "https://raw.githubusercontent.com/NeverSinkDev/NeverSink-Filter-for-PoE2/refs/heads/main/(STYLE)%20DARKMODE/NeverSink's%20filter%202%20-%203-STRICT%20(darkmode)%20.filter".to_string(),
        },
        Filter {
            name: Some("NeverSinks Filter Dark - 4 Very Strict".to_string()),
            url: "https://raw.githubusercontent.com/NeverSinkDev/NeverSink-Filter-for-PoE2/refs/heads/main/(STYLE)%20DARKMODE/NeverSink's%20filter%202%20-%204-VERY-STRICT%20(darkmode)%20.filter".to_string(),
        },
        Filter {
            name: Some("NeverSinks Filter Dark - 5 Uber Strict".to_string()),
            url: "https://raw.githubusercontent.com/NeverSinkDev/NeverSink-Filter-for-PoE2/refs/heads/main/(STYLE)%20DARKMODE/NeverSink's%20filter%202%20-%205-UBER-STRICT%20(darkmode)%20.filter".to_string(),
        },
        Filter {
            name: Some("NeverSinks Filter Dark - 6 Uber Plus Strict".to_string()),
            url: "https://raw.githubusercontent.com/NeverSinkDev/NeverSink-Filter-for-PoE2/refs/heads/main/(STYLE)%20DARKMODE/NeverSink's%20filter%202%20-%206-UBER-PLUS-STRICT%20(darkmode)%20.filter".to_string(),
        },
        // Pecham
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
