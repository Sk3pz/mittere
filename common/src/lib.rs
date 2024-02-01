use chrono::{DateTime, Local};

pub mod message;

pub fn to_local_time(time: &str) -> Result<DateTime<Local>, chrono::ParseError> {
    let time = DateTime::parse_from_rfc3339(time)?;
    Ok(time.with_timezone(&Local))
}