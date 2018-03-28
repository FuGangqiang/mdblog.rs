use std::collections::HashMap;
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
    /// blog build directory
    pub build_dir: String,
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
            build_dir: String::from("_build"),
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
        let mut map = HashMap::new();
        map.insert("theme".to_string(), self.theme.clone().into());
        map.insert("site_name".to_string(), self.site_name.clone().into());
        map.insert("site_motto".to_string(), self.site_motto.clone().into());
        map.insert("footer_note".to_string(), self.footer_note.clone().into());
        map.insert("build_dir".to_string(), self.build_dir.clone().into());
        map.insert("rebuild_interval".to_string(), self.rebuild_interval.clone().into());
        map.insert("url_prefix".to_string(), self.url_prefix.clone().into());
        Ok(map)
    }
}
