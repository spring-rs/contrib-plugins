use serde::Deserialize;
use spring::config::Configurable;

#[derive(Debug, Configurable, Clone, Deserialize)]
#[config_prefix = "utoipa"]
pub struct UtoipaConfig {
    #[serde(default = "default_path")]
    pub path: String,
    #[cfg(any(
        feature = "rapidoc",
        feature = "redoc",
        feature = "scalar",
        feature = "swagger-ui"
    ))]
    #[serde(default = "default_visualizer_path")]
    pub visualizer_path: String,
}

fn default_path() -> String {
    String::from("/openapi.json")
}

#[cfg(any(
    feature = "rapidoc",
    feature = "redoc",
    feature = "scalar",
    feature = "swagger-ui"
))]
fn default_visualizer_path() -> String {
    String::from("/openapi")
}
