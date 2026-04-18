use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub browser_ws_url: String,
    pub log_level: String,
    pub site_name: String,
    pub links_to_shuffle: Vec<String>,
}
