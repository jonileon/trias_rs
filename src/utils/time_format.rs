use chrono::Local;

pub fn get_date_time_now() -> String {
    Local::now().format("%Y-%m-%dT%H:%M:%S").to_string()
}
