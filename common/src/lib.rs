use chrono::{DateTime, Local};

pub mod message;

pub fn to_local_time<S: Into<String>>(time: S) -> Result<DateTime<Local>, chrono::ParseError> {
    let time = DateTime::parse_from_rfc3339(time.into().as_str())?;
    Ok(time.with_timezone(&Local))
}