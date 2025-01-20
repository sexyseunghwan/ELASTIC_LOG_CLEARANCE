use crate::common::*;

use crate::models::log_format::*;

use crate::utils_module::io_utils::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct LogConfigs {
    pub logformat: Vec<LogFormat>,
}

impl LogConfigs {
    pub fn new() -> Self {
        let config_file_path: String = get_config_file_path();
        let log_configs: LogConfigs = match read_toml_from_file::<LogConfigs>(&config_file_path) {
            Ok(log_configs) => log_configs,
            Err(e) => {
                error!(
                    "[Error][LogConfigs->new()] Failed to get information for 'LogConfigs'.: {:?}",
                    e
                );
                panic!(
                    "[Error][LogConfigs->new()] Failed to get information for 'LogConfigs'.: {:?}",
                    e
                )
            }
        };

        log_configs
    }
}
