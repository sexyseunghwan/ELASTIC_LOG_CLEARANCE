use crate::common::*;

use crate::service::log_service::*;
use crate::service::schedule_service::*;

use crate::models::group_log_format::*;
use crate::models::log_configs::*;

pub struct MainHandler<L: LogService, S: ScheduleService> {
    log_service: L,
    schedule_service: S,
}

impl<L: LogService, S: ScheduleService> MainHandler<L, S> {
    pub fn new(log_service: L, schedule_service: S) -> Self {
        Self {
            log_service,
            schedule_service,
        }
    }

    #[doc = "메인 스케쥴러 함수"]
    pub async fn main_schedule(&self) -> Result<(), anyhow::Error> {
        let schedule: Schedule = self.schedule_service.load_schedule_info();
        let mut upcoming: cron::ScheduleIterator<'_, Utc> = schedule.upcoming(chrono::Utc);
        
        loop {
            let next: DateTime<Utc> = match upcoming.next() {
                Some(next) => next,
                None => {
                    error!("[Error][main_task()] Failed to execute schedule");
                    continue;
                }
            };
            
            let now: DateTime<Utc> = chrono::Utc::now();

            if next > now {
                let duration_until_next: std::time::Duration = match (next - now).to_std() {
                    Ok(duration_until_next) => duration_until_next,
                    Err(e) => {
                        error!(
                            "[Error][main()] Failed to calculate 'duration_until_next': {:?}",
                            e
                        );
                        continue;
                    }
                };

                let sleep_until_time: Instant = Instant::now() + duration_until_next;
                sleep_until(sleep_until_time).await;
            }

            match self.main_task() {
                Ok(delete_cnt) => {
                    info!("Successfully clear log files.: {}", delete_cnt);
                },
                Err(e) => {
                    error!("[Error][main_schedule()] {:?}", e);
                    continue;
                }
            }
        }
    }

    #[doc = "Elasticsearch 의 로그 보존 처리를 위한 메인 함수"]
    fn main_task(&self) -> Result<usize, anyhow::Error> {

        /* 로그포멧관련 파일을 읽어와준다. */
        let log_configs: LogConfigs = LogConfigs::new();

        /* 디렉토리 경로별로 구분해준다. */
        let group_format_list: Vec<GroupLogFormat> =
            self.log_service.classify_log_format(log_configs);
        
        /* 총 파일 삭제 개수 */
        let mut delete_file_cnt: usize = 0;

        /* 디렉토리 경로별로 삭제작업을 진행한다. */
        for format in group_format_list {
            /* 삭제 리스트 반환 */
            let target_file_list: Vec<path::PathBuf> =
                self.log_service.get_match_log_list(&format)?;

            /* 삭제 수행 */
            match self.log_service.remove_file(&target_file_list) {
                Ok(_) => (),
                Err(e) => {
                    error!("{:?}", e);
                    continue;
                } 
            }

            delete_file_cnt += target_file_list.len();
        }

        Ok(delete_file_cnt)
    }

}
