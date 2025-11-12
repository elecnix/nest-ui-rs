use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HvacState {
    pub mode: HvacMode,
    pub fan_mode: FanMode,
    pub current_temp: f32,
    pub target_temp: Option<f32>,
    pub target_temp_low: Option<f32>,
    pub target_temp_high: Option<f32>,
    pub humidity: Option<f32>,
    pub is_heating: bool,
    pub is_cooling: bool,
    pub is_fan_running: bool,
    pub eco_mode: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, Serialize, Deserialize)]
pub enum HvacMode {
    Off,
    Heat,
    Cool,
    HeatCool,
    Eco,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, Serialize, Deserialize)]
pub enum FanMode {
    Auto,
    On,
    Circulate,
}

impl Default for HvacState {
    fn default() -> Self {
        Self {
            mode: HvacMode::Off,
            fan_mode: FanMode::Auto,
            current_temp: 20.0,
            target_temp: Some(21.0),
            target_temp_low: Some(18.0),
            target_temp_high: Some(24.0),
            humidity: None,
            is_heating: false,
            is_cooling: false,
            is_fan_running: false,
            eco_mode: false,
        }
    }
}

impl HvacState {
    pub fn set_mode(&mut self, mode: HvacMode) {
        self.mode = mode;
        if mode == HvacMode::Eco {
            self.eco_mode = true;
        } else {
            self.eco_mode = false;
        }
    }

    pub fn set_target_temp(&mut self, temp: f32) {
        match self.mode {
            HvacMode::Heat | HvacMode::Cool => {
                self.target_temp = Some(temp);
            }
            HvacMode::HeatCool => {
                // Adjust the range, keeping the spread
                if let (Some(low), Some(high)) = (self.target_temp_low, self.target_temp_high) {
                    let spread = high - low;
                    self.target_temp_low = Some(temp - spread / 2.0);
                    self.target_temp_high = Some(temp + spread / 2.0);
                }
            }
            _ => {}
        }
    }

    pub fn needs_heating(&self) -> bool {
        match self.mode {
            HvacMode::Heat => {
                if let Some(target) = self.target_temp {
                    self.current_temp < target - 0.5
                } else {
                    false
                }
            }
            HvacMode::HeatCool => {
                if let Some(low) = self.target_temp_low {
                    self.current_temp < low - 0.5
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    pub fn needs_cooling(&self) -> bool {
        match self.mode {
            HvacMode::Cool => {
                if let Some(target) = self.target_temp {
                    self.current_temp > target + 0.5
                } else {
                    false
                }
            }
            HvacMode::HeatCool => {
                if let Some(high) = self.target_temp_high {
                    self.current_temp > high + 0.5
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}
