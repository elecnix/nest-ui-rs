use anyhow::Result;
use log::info;
use nest_ui::{
    core::{Config, Event, EventResult, Layer},
    state::HvacState,
    ui::TemperatureDisplay,
};
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("Nest Thermostat UI - Rust Edition");
    info!("Starting up...");

    // Load configuration
    let config = Config::default();
    info!(
        "Display size: {}x{}",
        config.display.width, config.display.height
    );

    // Initialize HVAC state
    let mut hvac_state = HvacState::default();
    info!("HVAC state initialized: {:?}", hvac_state.mode);

    // Create temperature display layer
    let center_x = (config.display.width / 2) as i32;
    let center_y = (config.display.height / 2) as i32;
    let mut temp_display =
        TemperatureDisplay::new("temperature".to_string(), center_x, center_y, 150);

    info!("UI layers initialized");

    // Simulate temperature updates
    let start_time = Instant::now();
    let mut frame_count = 0u64;

    loop {
        let elapsed = start_time.elapsed();
        let dt = Duration::from_millis(16); // ~60 FPS

        // Simulate temperature reading (slowly varying)
        let simulated_temp = 20.0 + (elapsed.as_secs_f32() * 0.1).sin() * 3.0;
        hvac_state.current_temp = simulated_temp;

        // Send temperature update event
        let event = Event::TemperatureUpdate {
            celsius: simulated_temp,
        };

        if let EventResult::NeedsRedraw = temp_display.handle_event(&event)? {
            frame_count += 1;
            if frame_count % 60 == 0 {
                info!(
                    "Frame {}: Temp = {:.1}°C, Target = {:?}°C, Mode = {:?}",
                    frame_count, hvac_state.current_temp, hvac_state.target_temp, hvac_state.mode
                );
            }
        }

        // Update layer
        temp_display.update(dt)?;

        // Check HVAC logic
        if hvac_state.needs_heating() && !hvac_state.is_heating {
            info!("Starting heating");
            hvac_state.is_heating = true;
        } else if !hvac_state.needs_heating() && hvac_state.is_heating {
            info!("Stopping heating");
            hvac_state.is_heating = false;
        }

        // Sleep to maintain frame rate
        tokio::time::sleep(dt).await;

        // Exit after demo duration
        if elapsed.as_secs() > 30 {
            info!("Demo complete. Shutting down.");
            break;
        }
    }

    info!("Total frames rendered: {}", frame_count);
    Ok(())
}
