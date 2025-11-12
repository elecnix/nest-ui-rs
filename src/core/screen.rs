use anyhow::Result;
use async_trait::async_trait;
use std::time::Duration;

use super::display::Display;
use super::event::{Event, EventResult};
use super::layer::LayerManager;

/// Unique identifier for a screen
pub type ScreenId = String;

/// A screen is a collection of layers that represent a single UI state
#[async_trait]
pub trait Screen: Send + Sync {
    /// Get the screen's unique identifier
    fn id(&self) -> &ScreenId;

    /// Initialize the screen
    async fn init(&mut self) -> Result<()>;

    /// Get the layer manager for this screen
    fn layers(&self) -> &LayerManager;

    /// Get mutable layer manager
    fn layers_mut(&mut self) -> &mut LayerManager;

    /// Render the screen
    fn render(&self, display: &mut Display) -> Result<()> {
        self.layers().render(display)
    }

    /// Handle an event
    fn handle_event(&mut self, event: &Event) -> Result<EventResult> {
        self.layers_mut().handle_event(event)
    }

    /// Update the screen
    fn update(&mut self, dt: Duration) -> Result<()> {
        self.layers_mut().update(dt)
    }

    /// Called when screen becomes active
    async fn on_enter(&mut self) -> Result<()> {
        Ok(())
    }

    /// Called when screen becomes inactive
    async fn on_exit(&mut self) -> Result<()> {
        Ok(())
    }

    /// Cleanup resources
    async fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }
}

/// Screen manager for navigation between screens
pub struct ScreenManager {
    screens: Vec<Box<dyn Screen>>,
    current_screen_id: Option<ScreenId>,
    screen_stack: Vec<ScreenId>,
}

impl ScreenManager {
    pub fn new() -> Self {
        Self {
            screens: Vec::new(),
            current_screen_id: None,
            screen_stack: Vec::new(),
        }
    }

    pub fn add_screen(&mut self, screen: Box<dyn Screen>) {
        self.screens.push(screen);
    }

    pub fn get_screen(&self, id: &ScreenId) -> Option<&dyn Screen> {
        self.screens
            .iter()
            .find(|s| s.id() == id)
            .map(|s| s.as_ref())
    }

    pub fn get_screen_mut(&mut self, id: &ScreenId) -> Option<&mut Box<dyn Screen>> {
        self.screens.iter_mut().find(|s| s.id() == id)
    }

    pub fn current_screen(&self) -> Option<&dyn Screen> {
        self.current_screen_id
            .as_ref()
            .and_then(|id| self.get_screen(id))
    }

    pub fn current_screen_mut(&mut self) -> Option<&mut Box<dyn Screen>> {
        if let Some(id) = self.current_screen_id.clone() {
            self.get_screen_mut(&id)
        } else {
            None
        }
    }

    pub async fn transition_to(&mut self, screen_id: &ScreenId, push_to_stack: bool) -> Result<()> {
        // Exit current screen
        let current_id_clone = self.current_screen_id.clone();
        if let Some(current_id) = current_id_clone {
            if let Some(screen) = self.get_screen_mut(&current_id) {
                screen.on_exit().await?;
            }

            if push_to_stack {
                self.screen_stack.push(current_id);
            }
        }

        // Enter new screen
        if let Some(screen) = self.get_screen_mut(screen_id) {
            screen.on_enter().await?;
            self.current_screen_id = Some(screen_id.clone());
            Ok(())
        } else {
            anyhow::bail!("Screen not found: {}", screen_id)
        }
    }

    pub async fn go_back(&mut self) -> Result<bool> {
        if let Some(prev_screen_id) = self.screen_stack.pop() {
            self.transition_to(&prev_screen_id, false).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn render(&self, display: &mut Display) -> Result<()> {
        if let Some(screen) = self.current_screen() {
            screen.render(display)?;
        }
        Ok(())
    }

    pub fn handle_event(&mut self, event: &Event) -> Result<EventResult> {
        if let Some(screen) = self.current_screen_mut() {
            screen.handle_event(event)
        } else {
            Ok(EventResult::NotHandled)
        }
    }

    pub fn update(&mut self, dt: Duration) -> Result<()> {
        if let Some(screen) = self.current_screen_mut() {
            screen.update(dt)?;
        }
        Ok(())
    }
}

impl Default for ScreenManager {
    fn default() -> Self {
        Self::new()
    }
}
