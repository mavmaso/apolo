#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    // pub database: DatabaseSettings,
    // pub application_port: u16,
    pub teste: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let mut settings = config::Config::default();

    settings.merge(config::File::with_name("configuration"))?;

    settings.try_into()
}
