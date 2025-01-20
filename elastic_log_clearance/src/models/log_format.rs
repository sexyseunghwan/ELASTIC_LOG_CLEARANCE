use crate::common::*;

#[derive(Serialize, Deserialize, Debug, Getters, Clone)]
#[getset(get = "pub")]
pub struct LogFormat {
    pub log_path: String,
    pub log_format: String,
    pub log_retention_period: i64,
    pub log_extension: Vec<String>,
}
