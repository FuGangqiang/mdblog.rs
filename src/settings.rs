use std::collections::HashMap;

use config::{ConfigError, Source, Value};
use serde::{Deserialize, Serialize};

/// blog setting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    /// blog base url
    pub site_url: String,
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
    /// blog theme name
    pub theme: String,
    /// blog theme root directory
    pub theme_root_dir: String,
    /// blog rebuild interval
    pub rebuild_interval: u8,
    /// post count per index page
    pub posts_per_page: usize,
}

impl Default for Settings {
    fn default() -> Self {
        return Settings {
            site_url: String::from(""),
            site_name: String::from("Mdblog"),
            site_motto: String::from("Simple is Beautiful!"),
            footer_note: String::from("Keep It Simple, Stupid!"),
            media_dir: String::from("media"),
            build_dir: String::from("_build"),
            theme: String::from("simple"),
            theme_root_dir: String::from("_themes"),
            rebuild_interval: 2,
            posts_per_page: 20,
        };
    }
}

impl Source for Settings {
    fn clone_into_box(&self) -> Box<dyn Source + Send + Sync> {
        Box::new((*self).clone())
    }

    fn collect(&self) -> Result<HashMap<String, Value>, ConfigError> {
        let serialized = serde_json::to_string(&self).expect("settings serialized error");
        let map = serde_json::from_str::<HashMap<String, Value>>(&serialized).expect("settings deserialized error");
        Ok(map)
    }
}
