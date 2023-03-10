//! src/configuration.rs
use secrecy::{Secret, ExposeSecret};

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    /*
     * Add configuration values from a file named 'configuration'.
     * It will look for any top-level file with an extension
     * that 'config' knows how to parse: yaml, json, etc.
     */
    let settings = config::Config::builder().add_source(config::File::with_name("configuration"));

    /*
     * Try to convert the configuration values to read into
     * our Settings type below
     */
    match settings.build() {
        Ok(config) => config.try_deserialize::<Settings>(),
        Err(e) => Err(e), 
    }
}

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub test: TestSettings,
    pub application_port: u16
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

#[derive(serde::Deserialize)]
pub struct TestSettings {
    pub secret_key: String
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password.expose_secret(), self.host, self.port, self.database_name
        ))
    }
    pub fn connection_string_without_db(&self) -> Secret<String> {
        Secret::new(format!(
            "postgres://{}:{}@{}:{}",
            self.username, self.password.expose_secret(), self.host, self.port
        ))
    }
}

