#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct Settings {
    pub use_light_mode: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            use_light_mode: true,
        }
    }
}
