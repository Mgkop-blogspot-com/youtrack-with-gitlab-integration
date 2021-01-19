use config::{ConfigError, Value};
use config::FileFormat;
use config::Value as ConfigValue;
use std::collections::HashMap;
use std::collections::hash_map::RandomState;

lazy_static! {
	 pub static ref SETTINGS: config::Config = {
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

pub fn get_map(key: &str) -> Result<HashMap<String, String>, ConfigError> {
    SETTINGS.get_table(key)
        .map(|table| table.iter()
            .map(|(key, value)| (key.clone(), value.to_string())
            ).collect())
}

pub fn get_array<T>(key: &str) -> Result<Vec<T>, ConfigError> where T: From<config::Value> {
    SETTINGS.get_array(key)
        .map(|list| list.iter()
            .map(|value| T::from(value.clone()))
            .collect()
        )
}

pub fn get_label_definition(key: &str) -> Result<(String, String), Box<dyn std::error::Error>> {
    let default_style = "13".to_string();
    let label_name = SETTINGS.get_str(key).unwrap();
    let value = SETTINGS.get::<ConfigValue>(format!("labels.{label_name}", label_name = label_name).as_str()).unwrap();
    match (value.clone().into_str(), value.into_table()) {
        (Ok(title), Err(_)) => Ok((title, default_style)),
        (Err(_), Ok(map)) => {
            let title = map.get("title")
                .ok_or(ConfigError::Message("Key \"title\" in label configuration is required".to_string()))
                .and_then(|value| value.clone().into_str())
                .unwrap_or(format!(r###"Type of the `title` ({path}) is not string"###, path = key));

            let style_res = map.get("style")
                .and_then(|it| it.clone().into_int().ok())
                .map(|it| it.to_string());
            let style = style_res
                .unwrap_or(default_style);
            Ok((title, style))
        }
        _ => Err(box ConfigError::Message(format!("Undefined problem with configuration of hte label {}", key)))
    }
}
