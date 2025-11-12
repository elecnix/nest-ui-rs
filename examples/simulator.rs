use anyhow::Result;
use embedded_graphics::{draw_target::DrawTarget, pixelcolor::Rgb565, prelude::RgbColor};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorEvent, Window};
use log::info;
use nest_ui::{
    core::event::HvacDisplayMode,
    core::{new_display, Display, Event, Layer},
    state::{HvacMode, HvacState},
    ui::TemperatureDisplay,
};
use std::thread;
use std::time::{Duration, Instant};

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("Nest Thermostat UI - Visual Simulator");
    info!("Controls: Rotate mouse wheel to change temperature, ESC to quit");

    // Create display (480x480 like real Nest)
    let mut display: Display = new_display(480, 480);

    // Create window - use default theme for accurate color rendering
    let output_settings = OutputSettingsBuilder::new().scale(1).build();
    let mut window = Window::new("Nest Thermostat UI", &output_settings);

    // Initialize HVAC state - start with heating active
    let mut hvac_state = HvacState::default();
    hvac_state.mode = HvacMode::Heat;
    hvac_state.target_temp = Some(22.0);
    hvac_state.current_temp = 19.5; // Below target so heating activates
    hvac_state.is_heating = true;

    // Create UI layer
    let mut temp_display = TemperatureDisplay::new("temperature".into(), 240, 240, 180);
    temp_display.set_target_temp(hvac_state.target_temp);

    info!(
        "HVAC Mode: {:?}, Target: {:?}°C",
        hvac_state.mode, hvac_state.target_temp
    );

    let start_time = Instant::now();
    let mut last_update = Instant::now();
    let mut encoder_delta = 0i32;

    // Initial render (important for screenshot capture)
    temp_display.handle_event(&Event::TemperatureUpdate {
        celsius: hvac_state.current_temp,
    })?;
    temp_display.handle_event(&Event::HvacStateChanged {
        mode: HvacDisplayMode::Heat,
        is_heating: hvac_state.is_heating,
        is_cooling: hvac_state.is_cooling,
    })?;
    display.clear(Rgb565::BLACK).unwrap();
    info!("Rendering initial frame...");
    temp_display.render(&mut display)?;
    info!("Render complete, updating window...");
    window.update(&display);

    // Check if we're in screenshot mode - if so, do one more update and exit
    if std::env::var("EG_SIMULATOR_DUMP").is_ok() {
        info!("Screenshot mode detected - doing final update...");
        // Do a second update to ensure screenshot captures rendered content
        thread::sleep(Duration::from_millis(100));
        window.update(&display);
        info!("Screenshot captured, exiting");
        return Ok(());
    }

    'running: loop {
        // Handle window events
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::MouseWheel { scroll_delta, .. } => {
                    // Mouse wheel simulates encoder rotation
                    encoder_delta += scroll_delta.y as i32;
                    info!("Encoder delta: {}", scroll_delta.y);
                }
                SimulatorEvent::MouseButtonDown { mouse_btn, .. } => {
                    info!("Button pressed: {:?}", mouse_btn);
                    // Could simulate encoder press here
                }
                _ => {}
            }
        }

        // Process encoder events
        if encoder_delta != 0 {
            // Adjust target temperature
            hvac_state.target_temp =
                Some(hvac_state.target_temp.unwrap_or(21.0) + (encoder_delta as f32 * 0.5));
            temp_display.set_target_temp(hvac_state.target_temp);
            info!("Target temp adjusted to: {:?}°C", hvac_state.target_temp);
            encoder_delta = 0;
        }

        // Simulate temperature sensor (slow sine wave)
        if last_update.elapsed() > Duration::from_millis(100) {
            let elapsed = start_time.elapsed();
            let simulated_temp = 20.0 + (elapsed.as_secs_f32() * 0.1).sin() * 3.0;
            hvac_state.current_temp = simulated_temp;
            temp_display.handle_event(&Event::TemperatureUpdate {
                celsius: simulated_temp,
            })?;

            // Check if HVAC needs to activate
            if hvac_state.needs_heating() && !hvac_state.is_heating {
                info!(
                    "🔥 Starting heating - Temp: {:.1}°C < Target: {:?}°C",
                    hvac_state.current_temp, hvac_state.target_temp
                );
                hvac_state.is_heating = true;
            } else if !hvac_state.needs_heating() && hvac_state.is_heating {
                info!(
                    "✓ Stopping heating - Temp: {:.1}°C reached target",
                    hvac_state.current_temp
                );
                hvac_state.is_heating = false;
            }

            last_update = Instant::now();
        }

        // Update HVAC display layer
        temp_display.handle_event(&Event::HvacStateChanged {
            mode: match hvac_state.mode {
                HvacMode::Off => HvacDisplayMode::Off,
                HvacMode::Heat => HvacDisplayMode::Heat,
                HvacMode::Cool => HvacDisplayMode::Cool,
                HvacMode::HeatCool => HvacDisplayMode::HeatCool,
                HvacMode::Eco => HvacDisplayMode::Eco,
            },
            is_heating: hvac_state.is_heating,
            is_cooling: hvac_state.is_cooling,
        })?;

        // Clear display and render layer
        display.clear(Rgb565::BLACK).unwrap();
        temp_display.render(&mut display)?;

        // Small delay to cap frame rate
        thread::sleep(Duration::from_millis(16)); // ~60 FPS
    }

    info!("Simulator closed");
    Ok(())
}
