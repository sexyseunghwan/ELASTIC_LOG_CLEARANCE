use crate::common::*;

use crate::service::log_service::*;

use crate::models::group_log_format::*;
use crate::models::log_configs::*;

pub struct MainHandler<L: LogService> {
    log_service: L,
}

impl<L: LogService> MainHandler<L> {
    pub fn new(log_service: L) -> Self {
        Self { log_service }
    }

    #[doc = "Elasticsearch 의 로그 보존 처리를 위한 메인 함수"]
    pub async fn main_task(&self) -> Result<(), anyhow::Error> {
        /* 로그포멧관련 파일을 읽어와준다. */
        let log_configs: LogConfigs = LogConfigs::new();
        /* 디렉토리 경로별로 구분해준다. */
        let group_format_list: Vec<GroupLogFormat> =
            self.log_service.classify_log_format(log_configs);
        
        /* 디렉토리 경로별로 삭제작업을 진행한다. */
        for format in group_format_list {
            /* 삭제 리스트 반환 */
            let target_file_list: Vec<path::PathBuf> =
                self.log_service.get_match_log_list(&format)?;

            println!("{:?}", target_file_list);
            /* 삭제 수행 */

            self.log_service.remove_file(&target_file_list)?;
        }

        /* The END */
        Ok(())
    }
}
