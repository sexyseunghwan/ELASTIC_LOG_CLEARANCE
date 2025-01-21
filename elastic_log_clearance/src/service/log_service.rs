use crate::common::*;

use crate::models::group_log_format::*;
use crate::models::log_configs::*;
use crate::models::log_format::*;

use crate::utils_module::io_utils::*;
use crate::utils_module::time_utils::*;

pub trait LogService {
    fn classify_log_format(&self, log_configs: LogConfigs) -> Vec<GroupLogFormat>;
    fn get_match_log_list(
        &self,
        log_format: &GroupLogFormat,
    ) -> Result<Vec<path::PathBuf>, anyhow::Error>;
    fn is_matching_extension(
        &self,
        file: &path::Path,
        log_extensions: &[String],
    ) -> Result<bool, anyhow::Error>;
    fn extract_date_from_filename(&self, file_name: &str) -> Result<NaiveDate, anyhow::Error>;
    fn calculate_expiration_date(
        &self,
        file_date: NaiveDate,
        retention_period: i64,
    ) -> Result<NaiveDate, anyhow::Error>;
    fn check_file_by_rules(
        &self,
        file: path::PathBuf,
        log_format_list: &Vec<LogFormat>,
    ) -> Result<bool, anyhow::Error>;
    fn remove_file(&self, file_list: &Vec<path::PathBuf>) -> Result<(), anyhow::Error>;
}

#[derive(Clone, Debug, new)]
pub struct LogServicePub;

impl LogService for LogServicePub {
    #[doc = "삭제 대상로그를 분류해주는 함수"]
    /// # Arguments
    /// * `log_configs` - 삭제 대상 로그 정보가 들어있는 객체
    ///
    /// # Returns
    /// * Vec<GroupLogFormat>
    fn classify_log_format(&self, log_configs: LogConfigs) -> Vec<GroupLogFormat> {
        let mut log_hash_map: HashMap<String, Vec<LogFormat>> = HashMap::new();
        let mut group_list_format_list: Vec<GroupLogFormat> = Vec::new();

        for log_format in log_configs.logformat {
            let log_path: &String = log_format.log_path();

            log_hash_map
                .entry(log_path.clone())
                .or_insert(Vec::new())
                .push(log_format);
        }

        for (key, value) in &log_hash_map {
            let group_log_format: GroupLogFormat =
                GroupLogFormat::new(key.to_string(), value.to_vec());
            group_list_format_list.push(group_log_format);
        }

        group_list_format_list
    }

    #[doc = "파일의 확장자가 삭제대상 확장자와 부합한지 체크해주는 함수"]
    /// # Arguments
    /// * `file` - 대상 파일
    /// * `log_extensions` - 로그보존기간 리스트
    ///
    /// # Returns
    /// * Result<bool, anyhow::Error>
    fn is_matching_extension(
        &self,
        file: &path::Path,
        log_extensions: &[String],
    ) -> Result<bool, anyhow::Error> {
        /* 파라미터로 넘어온 파일의 확장자. */
        let file_extension: &ffi::OsStr = file.extension().ok_or_else(|| {
            anyhow!(
                "[Error][is_matching_extension()] Unknown file extension: {:?}",
                file
            )
        })?;

        /* 삭제 예약된 확장자중에 파라미터로 넘어온 파일과 같은 확장자가 있는지 확인해준다. */
        for extension in log_extensions {
            let extension_str: &str = extension.as_str();
            let file_extension_str: &str = file_extension.to_str().ok_or_else(|| anyhow!("[Error][is_matching_extension()] Problem converting 'file_extension_str' data."))?;

            if extension_str == file_extension_str {
                return Ok(true);
            }
        }

        Ok(false)
    }

    #[doc = "파일날짜에서 날짜데이터를 추출하는 함수"]
    /// # Arguments
    /// * `file_name` - 대상 파일 이름
    ///
    /// # Returns
    /// * Result<NaiveDate, anyhow::Error>
    fn extract_date_from_filename(&self, file_name: &str) -> Result<NaiveDate, anyhow::Error> {
        /* 파일 이름 내에서 날짜 형식을 따로 뽑아내기 위함. */
        let re: Regex = Regex::new(r"(\d{4})[-._](\d{2})[-._](\d{2})")?;

        /* 로그파일 날짜 뽑기 */
        let file_write_date: NaiveDate = re.captures(file_name).and_then(|cap| {
            let year: i32 = cap.get(1)?.as_str().parse::<i32>().ok()?;
            let month: u32 = cap.get(2)?.as_str().parse::<u32>().ok()?;
            let day: u32 = cap.get(3)?.as_str().parse::<u32>().ok()?;

            Some(get_naivedate(year, month, day))
        }).ok_or_else(|| anyhow!("[Error][check_file_by_rules()] 'file_write_date': Invalid content captured in regular expression."))?
        .map_err(|err| anyhow!("[Error][check_file_by_rules()] Date conversion failed.: {:?}", err))?;

        Ok(file_write_date)
    }

    #[doc = "파라미터로 넘어온 로그파일의 날짜를 추출해주는 함수"]
    /// # Arguments
    /// * `file_date` - 로그파일 작성 날짜
    /// * `retention_period` - 보존기간 정보
    ///
    /// # Returns
    /// * Result<NaiveDate, anyhow::Error>
    fn calculate_expiration_date(
        &self,
        file_date: NaiveDate,
        retention_period: i64,
    ) -> Result<NaiveDate, anyhow::Error> {
        file_date
            .checked_add_signed(chrono::Duration::days(retention_period))
            .ok_or_else(|| {
                anyhow!(
                    "[Error][check_file_by_rules()] Failed to add retention period to date: {}",
                    file_date
                )
            })
    }

    #[doc = "파라미터로 들어온 파일이 로그 포멧정보의 데이터 규칙에 부합하는지 체크해주는 함수."]
    /// # Arguments
    /// * `log_format` - 그룹별 로그 포멧정보
    ///
    /// # Returns
    /// * Result<bool, anyhow::Error>
    fn check_file_by_rules(
        &self,
        file: path::PathBuf,
        log_format_list: &Vec<LogFormat>,
    ) -> Result<bool, anyhow::Error> {
        for formatter in log_format_list {
            if !self.is_matching_extension(&file, formatter.log_extension())? {
                info!("The extension does not match.: {:?}", file);
                continue;
            }

            if let Some(file_name_str) = file.file_name().and_then(|name| name.to_str()) {
                /* 파일 이름 접두사 확인 */
                if !file_name_str.starts_with(formatter.log_format()) {
                    info!("The prefix does not match.: {:?}", file_name_str);
                    continue;
                }
                
                /* 로그파일이 보존기간 이내인지 확인 */
                let file_write_date: NaiveDate = self.extract_date_from_filename(file_name_str)?;
                let expiration_date: NaiveDate = self.calculate_expiration_date(
                    file_write_date,
                    *formatter.log_retention_period(),
                )?;

                let current_date: NaiveDate = get_current_kor_naivedate();

                if current_date > expiration_date {
                    return Ok(true);
                }
            } else {
                error!(
                    "[Error][check_file_by_rules()] Invalid file name: {:?}",
                    file
                );
                return Ok(false);
            }
        }

        Ok(false)
    }

    #[doc = "그룹별로 분류된 로그삭제대상 정보 중 진짜 삭제할 로그파일들의 경로를 벡터형식으로 반환"]
    /// # Arguments
    /// * `log_format` - 그룹별 로그 포멧정보
    ///
    /// # Returns
    /// * Result<Vec<path::PathBuf>, anyhow::Error>
    fn get_match_log_list(
        &self,
        log_formats: &GroupLogFormat,
    ) -> Result<Vec<path::PathBuf>, anyhow::Error> {
        let mut match_list: Vec<path::PathBuf> = Vec::new();

        /* 특정 디렉토리 하위에 있는 모든 폴더를 가져와준다. */
        let mon_file_dir: String = log_formats.group_path().to_string(); /* 상위 디렉토리 경로 */
        let watch_file_list: Vec<path::PathBuf> = read_all_files_in_dir(&mon_file_dir)?; /* 해당 디렉토리 하위에 있는 파일 리스트 */
        let log_format_list: &Vec<LogFormat> = log_formats.log_format_list();

        /* 특정 디렉토리 하위에 있는 모든 파일을 순회하면서 동작 */
        for file in &watch_file_list {
            let flag: bool = self.check_file_by_rules(file.clone(), log_format_list)?;
            if flag {
                match_list.push(file.clone());
            }
        }

        Ok(match_list)
    }

    #[doc = "넘겨진 파일 리스트를 모두 지워주는 함수"]
    /// # Arguments
    /// * `file_list` - 삭제할 파일 리스트
    ///
    /// # Returns
    /// * Result<(), anyhow::Error>
    fn remove_file(&self, file_list: &Vec<path::PathBuf>) -> Result<(), anyhow::Error> {
        for delete_file in file_list {
            if delete_file.exists() {
                /* 파일이 존재하면 제거 */
                fs::remove_file(&delete_file)?;
                info!("File successfully removed: {:?}", delete_file);
            } else {
                /* 파일이 존재하지 않으면 에러처리 */
                error!(
                    "[Error][remove_file()] File does not exist: {:?}",
                    delete_file
                );
            }
        }

        Ok(())
    }
}
