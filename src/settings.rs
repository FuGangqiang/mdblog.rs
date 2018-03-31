use std::collections::HashMap;
use serde_json;
use config::{Source, Value, ConfigError};

/// blog setting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// blog theme name
    pub theme: String,
    /// blog site name
    pub site_name: String,
    /// blog site motto
    pub site_motto: String,
    /// blog footer note
    pub footer_note: String,
    /// blog media directory
    pub media_dir: String,
    /// blog build root directory
    pub build_dir: String,
    /// blog theme root directory
    pub theme_root_dir: String,
    /// blog rebuild interval
    pub rebuild_interval: i64,
    /// blog url prefix
    pub url_prefix: String,
}

impl Default for Settings {
    fn default() -> Self {
        return Settings {
            theme: String::from("simple"),
            site_name: String::from("Mdblog"),
            site_motto: String::from("Simple is Beautiful!"),
            footer_note: String::from("Keep It Simple, Stupid!"),
            media_dir: String::from("media"),
            build_dir: String::from("_build"),
            theme_root_dir: String::from("_theme"),
            rebuild_interval: 2,
            url_prefix: Default::default(),
        }
    }
}

impl Source for Settings {
    fn clone_into_box(&self) -> Box<Source + Send + Sync> {
        Box::new((*self).clone())
    }

    fn collect(&self) -> Result<HashMap<String, Value>, ConfigError> {
        let serialized = serde_json::to_string(&self)
            .expect("settings serialized error");
        let map = serde_json::from_str::<HashMap<String, Value>>(&serialized)
            .expect("settings deserialized error");
        Ok(map)
    }
}
