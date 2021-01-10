use config::ConfigError;
use config::FileFormat;
use serde_json::Value;

lazy_static! {
	 static ref SETTINGS: config::Config = {
		  let mut settings = config::Config::default();
		  settings
				// Add in `./Settings.toml`
				.merge(config::File::with_name("settings").format(FileFormat::Yaml)).unwrap()
				// Add in settings from the environment (with a prefix of APP)
				// Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
				.merge(config::Environment::with_prefix("APP")).unwrap();
		  settings
	 };
}

pub fn get_str<'a>(key: &str) -> Result<String, ConfigError> {
    SETTINGS.get_str(key)
}

pub fn get_int(key: &str) -> Result<i64, ConfigError> {
    SETTINGS.get_int(key)
}

pub fn get_map<T>(key: &str) -> Result<Vec<(String, T)>, ConfigError> where T: From<config::Value> {
    SETTINGS.get_table(key)
        .map(|table| table.iter()
            .map(|(key, value)| (key.clone(), T::from(value.clone()))
            ).collect())
}

pub fn get_array<T>(key: &str) -> Result<Vec<T>, ConfigError> where T: From<config::Value> {
    SETTINGS.get_array(key)
        .map(|list| list.iter()
            .map(|value| T::from(value.clone()))
            .collect()
        )
}
