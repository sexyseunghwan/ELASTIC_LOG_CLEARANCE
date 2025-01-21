/*
Author      : Seunghwan Shin
Create date : 2025-01-21
Description : Elasticsearch 로그 저장주기 관리 해주는 프로그램

History     : 2025-01-21 Seunghwan Shin       # [v.1.0.0] first create
*/

mod common;
use common::*;

mod handlers;
use handlers::main_handler::*;

mod utils_module;
use utils_module::logger_utils::*;

mod models;

mod service;
use service::{log_service::*, schedule_service::*};

#[tokio::main]
async fn main() {
    /* 전역 로거 설정 */
    set_global_logger();
    info!("Elasticsearch log clear program start!");

    let log_service: LogServicePub = LogServicePub::new();
    let schedule_service: ScheduleServicePub = ScheduleServicePub::new();
    let main_handler: MainHandler<LogServicePub, ScheduleServicePub> =
        MainHandler::new(log_service, schedule_service);

    /* 메인 함수 실행 */
    match main_handler.main_schedule().await {
        Ok(_) => {
            info!("Successfully clear log files.")
        }
        Err(e) => {
            error!("[Error][main()] Failed to clear log files.: {:?}", e);
        }
    }
}