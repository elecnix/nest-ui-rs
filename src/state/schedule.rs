use chrono::{NaiveTime, Weekday};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Schedule {
    pub entries: Vec<ScheduleEntry>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleEntry {
    pub day: Weekday,
    pub time: NaiveTime,
    pub temp: f32,
    pub mode: String,
}

impl Default for Schedule {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            enabled: false,
        }
    }
}
