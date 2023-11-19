use config::{builder::DefaultState, ConfigBuilder, Environment};
use dotenv::dotenv;
use serde::Deserialize;
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions, PgSslMode},
    PgPool,
};

use crate::settings::AppEnv;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct DbSettings {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) database_name: String,
    pub(crate) require_ssl: bool,
}

impl DbSettings {
    pub(crate) fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
            .ssl_mode(ssl_mode)
    }

    pub(crate) fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.database_name)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct AppSettings {
    pub(crate) app_env: AppEnv,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Settings {
    pub(crate) db: DbSettings,
    // pub(crate) app: AppSettings,
}

pub(crate) fn get_pool(config: &DbSettings) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(config.with_db())
}

pub(crate) fn get_configuration() -> Result<Settings, config::ConfigError> {
    dotenv().ok();

    let app_env: AppEnv = std::env::var("APP_ENV")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENV");

    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let mut config_dir = base_path.join("configuration");

    // if app_env == AppEnv::Test || app_env == AppEnv::Local {
    //     config_dir = base_path.join("../configuration");
    // }

    let settings = ConfigBuilder::<DefaultState>::default()
        .add_source(config::File::from(config_dir.join("base")).required(true))
        .add_source(config::File::from(config_dir.join(app_env.to_string())).required(true))
        .add_source(Environment::with_prefix("dawgie").separator("__"));

    settings.build()?.try_deserialize()
}
