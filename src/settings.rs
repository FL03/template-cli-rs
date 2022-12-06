/*
    Appellation: settings <library>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/
use scsys::prelude::config::{Config, Environment};
use scsys::prelude::{try_collect_config_files, ConfigResult, Configurable, Logger, Server};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Settings {
    pub logger: Logger,
    pub server: Server,
}

impl Settings {
    pub fn build() -> ConfigResult<Self> {
        let mut builder = Config::builder()
            .add_source(Environment::default().separator("__"))
            .set_default("logger.level", "info")?
            .set_default("server.host", "127.0.0.1")?
            .set_default("server.port", 8080)?;
            if let Ok(f) = try_collect_config_files("**/*.config.*", false) {
                builder = builder.add_source(f);
            }
            if let Ok(lvl) = std::env::var("RUST_LOG") {
                builder = builder.set_override("logger.level", lvl)?;
            }
            if let Ok(port) = std::env::var("SERVER_PORT") {
                builder = builder.set_override("server.port", port)?;
            }

        builder.build()?.try_deserialize()
    }

    pub fn logger(&self) -> &Logger {
        &self.logger
    }

    pub fn server(&self) -> &Server {
        &self.server
    }
}

impl Configurable for Settings {
    type Settings = Self;

    fn settings(&self) -> &Self::Settings {
        self
    }
}

impl Default for Settings {
    fn default() -> Self {
        let d = Settings {
            logger: Default::default(),
            server: Server::new("127.0.0.1".to_string(), 8080),
        };
        Self::build().unwrap_or(d)
    }
}

impl std::fmt::Display for Settings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}