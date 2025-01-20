use crate::common::*;

use crate::models::log_format::*;

#[derive(Serialize, Deserialize, Debug, Clone, Getters, new)]
#[getset(get = "pub")]
pub struct GroupLogFormat {
    pub group_path: String,
    pub log_format_list: Vec<LogFormat>,
}
