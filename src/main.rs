use std::{fs::File, io::copy, path::PathBuf, str::FromStr};

use config::load_config;
use paths::find_filter_dir;
use reqwest::blocking::get;
use url::Url;

mod config;
mod paths;

fn main() -> Result<(), String> {
    let config = load_config().unwrap();
    let filter_dir = find_filter_dir().unwrap();

    let dir = PathBuf::from_str(filter_dir.as_str()).unwrap();

    println!("Found 'Path of Exile 2' filter directory: '{}'", filter_dir);

    for filter in config.filter.iter() {
        let name = match filter.name.clone() {
            Some(n) => format!("{} - by doryani.filter", n),
            None => {
                let url = Url::parse(filter.url.as_str()).unwrap();
                let segments = url.path_segments().unwrap();
                let seg = segments.last().unwrap().to_string();
                format!("{} - by doryani.filter", seg)
            }
        };

        let path = dir.join(name);

        if path.exists() {
            println!(
                "Filter: '{}' already exists, updating it...",
                path.to_str().unwrap().to_string()
            );
            // TODO: check if it needs an update
        }

        let response = get(filter.url.clone()).unwrap();

        if !response.status().is_success() {
            return Err(format!(
                "Failed to download: '{}' because: {}",
                filter.url.clone(),
                response.status()
            ));
        }

        let mut dest = File::create(path.to_str().unwrap().to_string()).unwrap();
        let content = response.bytes().unwrap();
        copy(&mut content.as_ref(), &mut dest).unwrap();

        println!("Wrote '{}'", path.to_str().unwrap());
    }

    Ok(())
}
