use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct AppSettings {}

#[derive(Debug, Clone, Deserialize, derive_more::Display, PartialEq)]
pub enum AppEnv {
    #[display(fmt = "local")]
    Local,
    #[display(fmt = "test")]
    Test,
    #[display(fmt = "staging")]
    Staging,
    #[display(fmt = "production")]
    Production,
}

impl TryFrom<String> for AppEnv {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "production" => Ok(Self::Production),
            "staging" => Ok(Self::Staging),
            "test" => Ok(Self::Test),
            "local" => Ok(Self::Local),
            other => Err(format!(
                "{other} is not a supported app_env. Use either `local`|`test`|`staging`|`production`"
            )),
        }
    }
}
