use anyhow::Result;
use async_trait::async_trait;

use crate::core::{
    display::Display,
    layer::{Layer, LayerId},
};

/// Ring menu layer - handles the circular menu interface
pub struct RingLayer {
    id: LayerId,
    visible: bool,
    enabled: bool,
}

impl RingLayer {
    pub fn new(id: LayerId) -> Self {
        Self {
            id,
            visible: true,
            enabled: true,
        }
    }
}

#[async_trait]
impl Layer for RingLayer {
    fn id(&self) -> &LayerId {
        &self.id
    }

    fn render(&self, _display: &mut Display) -> Result<()> {
        // TODO: Implement ring menu rendering
        Ok(())
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
