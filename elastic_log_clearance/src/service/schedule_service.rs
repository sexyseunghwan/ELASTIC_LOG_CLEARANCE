use crate::common::*;

pub trait ScheduleService {
    fn load_schedule_info(&self) -> Schedule;
}

#[derive(Clone, Debug, new)]
pub struct ScheduleServicePub;

impl ScheduleService for ScheduleServicePub {
    #[doc = "스케쥴 정보를 로드해주는 함수"]
    fn load_schedule_info(&self) -> Schedule {
        let start_cron: String = get_start_cron();

        let schedule: Schedule = match Schedule::from_str(&start_cron) {
            Ok(schedule) => schedule,
            Err(e) => {
                error!("[Error][load_schedule_info()] {:?}", e);
                panic!("{:?}", e)
            }
        };

        schedule
    }
}
