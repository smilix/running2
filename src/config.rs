use dotenv::dotenv;
use lazy_static::lazy_static;

#[derive(Clone, Deserialize, Debug)]
pub struct Config {
    pub database_file: String,
    pub rust_log: String,
    pub server: String,

    pub jwt_expiration_in_h: i64,
    pub jwt_key: String,

    pub login: String,
    // TODO: encrypt this
    pub password: String,
    // pub rust_backtrace: u8,

    pub static_webapp_folder: String,
}

// Throw the Config struct into a CONFIG lazy_static to avoid multiple processing
lazy_static! {
    pub static ref CONFIG: Config = get_config();
}

/// Use envy to inject dotenv and env vars into the Config struct
fn get_config() -> Config {
    dotenv().ok();

    match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("Configuration Error: {:#?}", error),
    }
}
