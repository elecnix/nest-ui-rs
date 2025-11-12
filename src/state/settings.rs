use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub device_name: String,
    pub location: String,
    pub temperature_unit: TemperatureUnit,
    pub display_brightness: u8,
    pub auto_away_enabled: bool,
    pub learning_enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            device_name: "Nest Thermostat".to_string(),
            location: "Home".to_string(),
            temperature_unit: TemperatureUnit::Celsius,
            display_brightness: 80,
            auto_away_enabled: true,
            learning_enabled: true,
        }
    }
}
