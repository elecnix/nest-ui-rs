use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub display: DisplayConfig,
    pub hvac: HvacConfig,
    pub network: NetworkConfig,
    pub system: SystemConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub width: u32,
    pub height: u32,
    pub brightness: u8,
    pub fade_up_time_ms: u64,
    pub fade_down_time_ms: u64,
    pub sleep_timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HvacConfig {
    pub temperature_unit: TemperatureUnit,
    pub min_heat_setpoint: f32,
    pub max_heat_setpoint: f32,
    pub min_cool_setpoint: f32,
    pub max_cool_setpoint: f32,
    pub safety_temp_min: f32,
    pub safety_temp_max: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub wifi_enabled: bool,
    pub cloud_sync_enabled: bool,
    pub api_endpoint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    pub device_name: String,
    pub location: String,
    pub language: String,
    pub timezone: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            display: DisplayConfig {
                width: 480,
                height: 480,
                brightness: 80,
                fade_up_time_ms: 300,
                fade_down_time_ms: 500,
                sleep_timeout_seconds: 30,
            },
            hvac: HvacConfig {
                temperature_unit: TemperatureUnit::Celsius,
                min_heat_setpoint: 10.0,
                max_heat_setpoint: 30.0,
                min_cool_setpoint: 16.0,
                max_cool_setpoint: 35.0,
                safety_temp_min: 5.0,
                safety_temp_max: 40.0,
            },
            network: NetworkConfig {
                wifi_enabled: true,
                cloud_sync_enabled: true,
                api_endpoint: "https://api.nest.com".to_string(),
            },
            system: SystemConfig {
                device_name: "Nest Thermostat".to_string(),
                location: "Home".to_string(),
                language: "en".to_string(),
                timezone: "UTC".to_string(),
            },
        }
    }
}

impl Config {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = std::fs::read_to_string(path)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let contents = toml::to_string_pretty(self)?;
        std::fs::write(path, contents)?;
        Ok(())
    }
}
