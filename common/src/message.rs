use std::error::Error;
use std::fmt::{Display, Formatter};
use send_it::Segment;

#[derive(Debug, Clone)]
pub enum MessageError {
    InvalidSegmentCount,
}

impl Display for MessageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid segment count")
    }
}

impl Error for MessageError {}

#[derive(Debug, Clone)]
pub struct Message {
    pub message: String,
    pub author: String,
    pub timestamp: String,
}

impl Message {

    pub fn new(message: String, author: String) -> Self {
        Self {
            message,
            author,
            timestamp: format!("{}", chrono::Local::now()),
        }
    }

    pub fn from_segments(segments: Vec<Segment>) -> Result<Self, MessageError> {
        if segments.len() != 3 {
            return Err(MessageError::InvalidSegmentCount);
        }

        let message = segments[0].to_string();
        let author = segments[1].to_string();
        let timestamp = segments[2].to_string();

        Ok(Self {
            message,
            author,
            timestamp,
        })
    }

    pub fn segmented(&self) -> Vec<Segment> {
        let mut segments = Vec::new();
        segments.push(Segment::from(self.message.clone()));
        segments.push(Segment::from(self.author.clone()));
        segments.push(Segment::from(self.timestamp.clone()));
        segments
    }

}