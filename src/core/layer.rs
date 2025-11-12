use anyhow::Result;
use async_trait::async_trait;
use std::time::Duration;

use super::display::Display;
use super::event::{Event, EventResult};

/// Unique identifier for a layer
pub type LayerId = String;

/// Base trait for all UI layers
///
/// Layers are composable UI elements that can be stacked and rendered.
/// They handle their own rendering, event processing, and updates.
#[async_trait]
pub trait Layer: Send + Sync {
    /// Get the layer's unique identifier
    fn id(&self) -> &LayerId;

    /// Initialize the layer
    async fn init(&mut self) -> Result<()> {
        Ok(())
    }

    /// Render the layer to the display
    fn render(&self, display: &mut Display) -> Result<()>;

    /// Handle an event
    fn handle_event(&mut self, _event: &Event) -> Result<EventResult> {
        Ok(EventResult::NotHandled)
    }

    /// Update the layer state (called periodically)
    fn update(&mut self, _dt: Duration) -> Result<()> {
        Ok(())
    }

    /// Check if the layer is visible
    fn is_visible(&self) -> bool {
        true
    }

    /// Set layer visibility
    fn set_visible(&mut self, visible: bool);

    /// Check if the layer is enabled (can receive events)
    fn is_enabled(&self) -> bool {
        true
    }

    /// Set layer enabled state
    fn set_enabled(&mut self, enabled: bool);

    /// Get layer z-order (higher = on top)
    fn z_order(&self) -> i32 {
        0
    }

    /// Called when layer is shown
    async fn on_show(&mut self) -> Result<()> {
        Ok(())
    }

    /// Called when layer is hidden
    async fn on_hide(&mut self) -> Result<()> {
        Ok(())
    }

    /// Cleanup resources
    async fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }
}

/// Layer manager for handling multiple layers
pub struct LayerManager {
    layers: Vec<Box<dyn Layer>>,
}

impl LayerManager {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn add_layer(&mut self, layer: Box<dyn Layer>) {
        self.layers.push(layer);
        self.sort_layers();
    }

    pub fn remove_layer(&mut self, id: &LayerId) -> Option<Box<dyn Layer>> {
        if let Some(pos) = self.layers.iter().position(|l| l.id() == id) {
            Some(self.layers.remove(pos))
        } else {
            None
        }
    }

    pub fn get_layer(&self, id: &LayerId) -> Option<&dyn Layer> {
        self.layers
            .iter()
            .find(|l| l.id() == id)
            .map(|l| l.as_ref())
    }

    pub fn get_layer_mut(&mut self, id: &LayerId) -> Option<&mut Box<dyn Layer>> {
        self.layers.iter_mut().find(|l| l.id() == id)
    }

    fn sort_layers(&mut self) {
        self.layers.sort_by_key(|l| l.z_order());
    }

    pub fn render(&self, display: &mut Display) -> Result<()> {
        for layer in &self.layers {
            if layer.is_visible() {
                layer.render(display)?;
            }
        }
        Ok(())
    }

    pub fn handle_event(&mut self, event: &Event) -> Result<EventResult> {
        // Process layers in reverse z-order (top to bottom)
        for layer in self.layers.iter_mut().rev() {
            if layer.is_enabled() && layer.is_visible() {
                match layer.handle_event(event)? {
                    EventResult::NotHandled => continue,
                    result => return Ok(result),
                }
            }
        }
        Ok(EventResult::NotHandled)
    }

    pub fn update(&mut self, dt: Duration) -> Result<()> {
        for layer in &mut self.layers {
            if layer.is_visible() {
                layer.update(dt)?;
            }
        }
        Ok(())
    }
}

impl Default for LayerManager {
    fn default() -> Self {
        Self::new()
    }
}
