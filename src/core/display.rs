use embedded_graphics::{pixelcolor::Rgb565, prelude::Size};
pub use embedded_graphics_simulator::SimulatorDisplay as EgSimulatorDisplay;

/// Primary display type used across the project (480x480 RGB565 framebuffer)
pub type Display = EgSimulatorDisplay<Rgb565>;

/// Helper to create a new simulator display with the desired dimensions
pub fn new_display(width: u32, height: u32) -> Display {
    EgSimulatorDisplay::new(Size::new(width, height))
}
