/*
Author      : Seunghwan Shin
Create date : 2025-00-00
Description : Elasticsearch 로그 저장주기 관리 해주는 프로그램

History     : 2025-01-24 Seunghwan Shin       # [v.1.0.0] first create
*/

// albaelastic-2021-09-07-1.json.gz
// albaelastic-2021-09-07-1.log.gz

/* 
albaelastic-2021-09-07-1.json.gz
albaelastic-2021.09.07.1.json.gz
albaelastic-2021_09_07_1.json.gz
*/

mod common;
use common::*;

mod handlers;
use handlers::main_handler::*;

mod utils_module;
use utils_module::logger_utils::*;

mod models;

mod service;
use service::log_service::*;
// mod controller;
// use controller::main_controller::*;

// mod service;
// use service::es_query_service::*;
// use service::query_service::*;

#[tokio::main]
async fn main() {
    /* 전역 로거 설정 */
    set_global_logger();

    let log_service: LogServicePub = LogServicePub::new();
    let main_handler: MainHandler<LogServicePub> = MainHandler::new(log_service);

    match main_handler.main_task().await {
        Ok(_) => {
            info!("Successfully clear log files.")
        }
        Err(e) => {
            error!("[Error][main()] Failed to clear log files.: {:?}", e);
        }
    }
}
