use anyhow::anyhow;

use crate::common::*;

use crate::models::group_log_format::*;
use crate::models::log_configs::*;
use crate::models::log_format::*;

use crate::utils_module::io_utils::*;
use crate::utils_module::time_utils::get_naivedate;

#[async_trait]
pub trait LogService {
    fn classify_log_format(&self, log_configs: LogConfigs) -> Vec<GroupLogFormat>;
    fn get_match_log_list(&self, log_format: &GroupLogFormat)
        -> Result<Vec<String>, anyhow::Error>;
    fn check_file_by_rules(
        &self,
        file: path::PathBuf,
        log_format_list: &Vec<LogFormat>
    ) -> Result<bool, anyhow::Error>;
}

#[derive(Clone, Debug, new)]
pub struct LogServicePub;

#[async_trait]
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

    #[doc = "파라미터로 들어온 파일이 로그 포멧정보의 데이터 규칙에 부합하는지 체크해주는 함수."]
    /// # Arguments
    /// * `log_format` - 그룹별 로그 포멧정보
    ///
    /// # Returns
    /// * Vec<GroupLogFormat>
    fn check_file_by_rules(
        &self,
        file: path::PathBuf,
        log_format_list: &Vec<LogFormat>
    ) -> Result<bool, anyhow::Error> {
        let mut total_result: bool = false;
        
        for formatter in log_format_list {

            let format: String = formatter.log_format().to_string();
            let retention_period: &usize = formatter.log_retention_period();
            let log_extension: &Vec<String> = formatter.log_extension();

            let file_extension: &ffi::OsStr = file.extension().ok_or_else(|| anyhow!("[Error][check_file_by_rules()] The extension of this file is unknown. : {:?}", file))?;
            
            let mut extenstion_flag: bool = false;

            for extension in log_extension {
    
                let extension_clone: String = extension.clone();
                if *extension_clone == *file_extension {
                    extenstion_flag = true;
                    break;
                }
            }
            
            /* 확장자 형식에 맞지 않는다면 불일치 처리해서 다음 루프로 이동 */
            if !extenstion_flag {
                continue;
            }
            
            /* 파일이름 접두사, 로그보존기간 확인 */
            if let Some(file_name_str) = file.file_name().and_then(|name| name.to_str()) {
                
                /* 파일 이름 접두사 확인 */
                if !file_name_str.starts_with(&format) {
                    return Ok(total_result)
                }
                
                /* 파일 이름 내에서 날짜 형식을 따로 뽑아내기 위함. */
                let re: Regex = Regex::new(r"(\d{4})[-._](\d{2})[-._](\d{2})")?;
                
                /* 로그작성 날짜 뽑기 */
                let test = re.captures(file_name_str).unwrap();

                println!("test: {:?}", test);
                println!("test: {:?}", test.get(0));
                println!("test: {:?}", test.get(1));
                println!("test: {:?}", test.get(2));
                println!("test: {:?}", test.get(3));
                
                // let file_write_date = re.captures(file_name_str).and_then(|cap| {
                //     let year: &str = cap.get(1)?.as_i32();
                //     let month: &str = cap.get(2)?.as_str();
                //     let day: &str = cap.get(3)?.as_str();

                //     // let date = get_naivedate(year, month, date);

                //     //Some(format!("{}-{}-{}", year, month, day))
                // }).ok_or_else(|| anyhow!("[Error][check_file_by_rules()] 'file_write_date': Invalid content captured in regular expression."))?;

                //let file_write_date_naive = 


            } else {   
                error!("[Error][check_file_by_rules()] There was a problem converting the file name."); 
                return Ok(total_result)
            }
            

        }



        
        Ok(total_result)
    }

    #[doc = "그룹별로 분류된 로그삭제대상 정보 중 진짜 삭제할 로그파일들의 경로를 벡터형식으로 반환"]
    /// # Arguments
    /// * `log_format` - 그룹별 로그 포멧정보
    ///
    /// # Returns
    /// * Vec<GroupLogFormat>
    fn get_match_log_list(
        &self,
        log_formats: &GroupLogFormat,
    ) -> Result<Vec<String>, anyhow::Error> {
        let mut match_list: Vec<String> = Vec::new();

        /* 특정 디렉토리 하위에 있는 모든 폴더를 가져와준다. */
        let watch_file_list: Vec<path::PathBuf> = read_all_files_in_dir(log_formats.group_path())?;
        let log_format_list: &Vec<LogFormat> = log_formats.log_format_list();
        
        /* 특정 디렉토리 하위에 있는 모든 파일을 순회하면서 동작 */
        for file in watch_file_list {
            self.check_file_by_rules(file, log_format_list);
        }

        Ok(match_list)
    }
}

// if let Some(file_name) = file.file_name().and_then(|name| name.to_str()) {
//     info!("filename: {}", file_name);
// }

// let file_extension = file
//     .extension()
//     .and_then(|ext| ext.to_str())
//     .map(|ext| ext.to_string())
//     .unwrap();

// info!("file_extension: {}", file_extension);

// for log_format in log_formats.log_format_list() {
//     // let format: &String = log_format.log_format();
//     // let retention_period: &usize = log_format.log_retention_period();
//     // let log_extension: &Vec<String> = log_format.log_extension();

//     //let file_name =

// }
