use config;
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize, PartialEq, Debug)]
pub struct Settings {
    pub application_port: u16,
    pub database: DatabaseSettings,
}

#[derive(Deserialize, PartialEq, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::from(Path::new("configuration.toml")))
        .build()?;

    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_configuration_match_values_of_file() {
        // Arrange
        let expected_settings = Settings {
            application_port: 8080,
            database: DatabaseSettings {
                username: "postgres".to_string(),
                password: "password".to_string(),
                port: 5432,
                host: "localhost".to_string(),
                database_name: "newsletter".to_string(),
            },
        };

        // Act
        let settings = get_configuration().expect("Failed to read configuration");

        // Assert
        assert_eq!(settings, expected_settings);
    }
}
