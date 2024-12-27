use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
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
    ]
}

pub fn load_config() -> Result<Config, String> {
    // TODO: actually add support for custom configs
    // TODO: get default path
    // TODO: if exists, read

    let config = Config { filter: defaults() };

    Ok(config)
}
