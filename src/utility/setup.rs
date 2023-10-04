use std::process::exit;

use crate::repository::mysql::{AppState, initialize_pool};
use crate::utility::configuration::Configuration;
use crate::utility::logger::logger;

pub async fn setup() -> (AppState, Configuration) {
    match logger() {
        Ok(_) => {
            log::info!("Logger configuration completed");
        }
        Err(_) => {
            log::error!("Logger configuration failed. Try to restart application");
            exit(0);
        }
    };

    let config = match Configuration::parse() {
        Ok(config) => config,
        Err(_) => {
            log::error!("Configuration parse failed. Check for missing env variables");
            exit(0);
        }
    };

    let app_state = match initialize_pool(config.db_url.clone(), config.db_pool_size.clone()).await {
        Ok(app_state) => {
            log::info!("Created mysql pool with {} max connections", config.db_pool_size);

            app_state
        }
        Err(_) => {
            log::error!("Mysql pool initialization failed. Check the database url for wrong data");
            exit(0);
        }
    };

    log::info!("Starting listener on http://{}:{}", config.ip, config.port);

    (app_state, config)
}