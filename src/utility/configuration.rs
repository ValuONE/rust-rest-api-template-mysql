use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Configuration {
    #[serde(default = "default_ip")]
    pub ip: String,
    #[serde(default = "default_port")]
    pub port: u16,
    pub db_url: String,
    #[serde(default = "default_db_pool_size")]
    pub db_pool_size: u32,
}

impl Configuration {
    pub fn parse() -> envy::Result<Configuration> {
        envy::from_env::<Self>()
    }
}

fn default_ip() -> String {
    "127.0.0.1".into()
}

fn default_port() -> u16 {
    4000
}

fn default_db_pool_size() -> u32 {
    10
}