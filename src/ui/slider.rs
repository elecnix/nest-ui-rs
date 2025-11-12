use crate::core::{
    display::Display,
    event::{Event, EventResult},
    layer::{Layer, LayerId},
};
use anyhow::Result;
use async_trait::async_trait;

/// Temperature slider for adjusting temperature
pub struct TemperatureSlider {
    id: LayerId,
    visible: bool,
    enabled: bool,
    value: f32,
    min: f32,
    max: f32,
}

impl TemperatureSlider {
    pub fn new(id: LayerId, min: f32, max: f32) -> Self {
        let value = (min + max) / 2.0;
        Self {
            id,
            visible: true,
            enabled: true,
            value,
            min,
            max,
        }
    }

    pub fn get_value(&self) -> f32 {
        self.value
    }

    pub fn set_value(&mut self, value: f32) {
        self.value = value.clamp(self.min, self.max);
    }

    pub fn adjust(&mut self, delta: f32) {
        self.value = (self.value + delta).clamp(self.min, self.max);
    }
}

#[async_trait]
impl Layer for TemperatureSlider {
    fn id(&self) -> &LayerId {
        &self.id
    }

    fn render(&self, _display: &mut Display) -> Result<()> {
        // TODO: Implement slider rendering
        Ok(())
    }

    fn handle_event(&mut self, event: &Event) -> Result<EventResult> {
        match event {
            Event::EncoderRotate { delta } => {
                // Adjust by 0.5 degrees per click
                self.adjust(*delta as f32 * 0.5);
                Ok(EventResult::NeedsRedraw)
            }
            _ => Ok(EventResult::NotHandled),
        }
    }

    fn is_visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}
