use std::{
    env,
    path::{Path, PathBuf},
    str::FromStr,
};

fn find_linux_steam_dir() -> Result<String, String> {
    let root_dirs = vec![
        "~/.steam/steam",
        "~/.var/app/com.valvesoftware.Steam/data/Steam",
    ];

    for dir in root_dirs.iter() {
        let expanded = shellexpand::tilde(dir).into_owned();
        let p = Path::new(&expanded);
        if p.exists() {
            return Ok(p.to_str().unwrap().to_string());
        }
    }

    Err("could not find steam directory".to_string())
}

pub fn find_filter_dir() -> Result<String, String> {
    match env::consts::OS {
        "linux" => {
            let steam_dir = find_linux_steam_dir().unwrap();

            let poe2_dir = PathBuf::from_str(steam_dir.as_str())
                .unwrap()
                .join("steamapps")
                .join("compatdata")
                .join("2694490")
                .join("pfx")
                .join("drive_c")
                .join("users")
                .join("steamuser")
                .join("My Documents")
                .join("My Games")
                .join("Path of Exile 2");

            if !poe2_dir.exists() {
                return Err(
                    "Path of Exile 2 could not be found, try installing it or starting it first"
                        .to_string(),
                );
            }

            Ok(poe2_dir.to_str().unwrap().to_string())
        }
        // "windows" => Err("windows is not yet supported".to_string()),
        os => Err(format!("OS: {} is not supported", os)),
    }
}
