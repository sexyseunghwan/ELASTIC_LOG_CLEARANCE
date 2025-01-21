use crate::common::*;

use crate::utils_module::io_utils::*;

static SERVER_CONFIG: once_lazy<Arc<ServerConfig>> = once_lazy::new(|| initialize_server_config());

#[derive(Serialize, Deserialize, Debug, Getters)]
#[getset(get = "pub")]
pub struct ServerConfig {
    pub config_file_path: String,
    pub start_cron: String,
}

#[doc = "가장 기본적인 server 의 설정파일을 초기화시켜준는 함수"]
fn initialize_server_config() -> Arc<ServerConfig> {
    let server_config: ServerConfig = match read_toml_from_file::<ServerConfig>(
        "./configs/server_config.toml",
    ) {
        Ok(server_config) => server_config,
        Err(e) => {
            error!("[Error][initialize_server_config()] Failed to get information 'server_config'.: {:?}", e);
            panic!("[Error][initialize_server_config()] Failed to get information 'server_config'.: {:?}", e)
        }
    };

    Arc::new(server_config)
}

#[doc = "server_config 데이터를 전역적으로 쓰기 위한 함수 - config_file_path"]
pub fn get_config_file_path() -> String {
    let server_config: &once_lazy<Arc<ServerConfig>> = &SERVER_CONFIG;
    server_config.config_file_path().to_string()
}

#[doc = "server_config 데이터를 전역적으로 쓰기 위한 함수 - start_cron"]
pub fn get_start_cron() -> String {
    let server_config: &once_lazy<Arc<ServerConfig>> = &SERVER_CONFIG;
    server_config.start_cron().to_string()
}
