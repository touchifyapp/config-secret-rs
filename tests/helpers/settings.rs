use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct Settings {
    pub server: ServerSettings,
    pub redis: RedisSettings,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RedisSettings {
    pub nodes: Vec<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ScopedSettings {
    pub a: Settings,
}
