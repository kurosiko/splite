use std::{fs::OpenOptions, io::Write};

use chrono::Local;

#[derive(Debug)]
pub enum LogType{
    WinApiError { message: String, code: u32},
    
}

impl LogType {
    fn log_format(&self)->String{
        match self {
            LogType::WinApiError { message, code } => format!("[WinApiError] 0x{:X} {}",code,message)
            
        }
    }
    
}


pub fn write_to_log(report:LogType){
    let date_now = Local::now().format("%Y%m%d%H%M%S").to_string();
    let mut log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("{}.log",date_now))
        .expect("log file cannot open. check the permission or else.");
    log_file.write_all(
        &report.log_format().into_bytes()
    );
}