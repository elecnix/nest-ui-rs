use anyhow::Result;
use async_trait::async_trait;
use embedded_graphics::{
    mono_font::{ascii::FONT_10X20, MonoTextStyle},
    pixelcolor::Rgb565,
    prelude::{Angle, Point, Primitive, RgbColor, WebColors},
    primitives::{Arc, Circle, PrimitiveStyle, PrimitiveStyleBuilder},
    text::{Alignment, Text},
    Drawable,
};

use crate::core::display::Display;
use crate::core::{
    event::{Event, EventResult, HvacDisplayMode},
    layer::{Layer, LayerId},
};
use std::time::Duration;

/// Temperature display widget
///
/// Displays the current temperature in a circular layout,
/// mimicking the Nest's iconic ring interface
pub struct TemperatureDisplay {
    id: LayerId,
    visible: bool,
    enabled: bool,
    current_temp: f32,
    target_temp: Option<f32>,
    unit: TemperatureUnit,
    center: Point,
    radius: u32,
    hvac_mode: HvacDisplayMode,
    is_heating: bool,
    is_cooling: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

impl TemperatureDisplay {
    pub fn new(id: LayerId, center_x: i32, center_y: i32, radius: u32) -> Self {
        Self {
            id,
            visible: true,
            enabled: true,
            current_temp: 20.0,
            target_temp: None,
            unit: TemperatureUnit::Celsius,
            center: Point::new(center_x, center_y),
            radius,
            hvac_mode: HvacDisplayMode::Off,
            is_heating: false,
            is_cooling: false,
        }
    }

    pub fn set_current_temp(&mut self, temp: f32) {
        self.current_temp = temp;
    }

    pub fn set_target_temp(&mut self, temp: Option<f32>) {
        self.target_temp = temp;
    }

    pub fn set_unit(&mut self, unit: TemperatureUnit) {
        self.unit = unit;
    }

    fn format_temp(&self, temp: f32) -> String {
        let display_temp = match self.unit {
            TemperatureUnit::Celsius => temp,
            TemperatureUnit::Fahrenheit => temp * 9.0 / 5.0 + 32.0,
        };
        format!("{:.1}°", display_temp)
    }

    fn unit_symbol(&self) -> &str {
        match self.unit {
            TemperatureUnit::Celsius => "C",
            TemperatureUnit::Fahrenheit => "F",
        }
    }

    fn temp_fraction(&self, temp_celsius: f32) -> f32 {
        const MIN_TEMP: f32 = 10.0;
        const MAX_TEMP: f32 = 30.0;
        ((temp_celsius - MIN_TEMP) / (MAX_TEMP - MIN_TEMP)).clamp(0.0, 1.0)
    }
}

#[async_trait]
impl Layer for TemperatureDisplay {
    fn id(&self) -> &LayerId {
        &self.id
    }

    fn render(&self, display: &mut Display) -> Result<()> {
        // Draw outer circle (the "ring")
        let diameter = self.radius * 2;
        let top_left = Point::new(
            self.center.x - self.radius as i32,
            self.center.y - self.radius as i32,
        );
        let _outer_circle = Circle::new(top_left, diameter);

        // Refined polished bezel - thinner, smoother like authentic Nest
        // Outer highlight (subtle)
        let outer_highlight = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb565::new(0x18, 0x20, 0x18))
            .stroke_width(1)
            .build();
        Circle::new(top_left, diameter)
            .into_styled(outer_highlight)
            .draw(display)
            .map_err(|_| anyhow::anyhow!("Failed to draw outer highlight"))?;

        // Main bezel - single refined ring
        let bezel_style = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb565::CSS_GRAY)
            .stroke_width(3)
            .build();
        Circle::new(
            Point::new(
                self.center.x - (self.radius as i32 - 1),
                self.center.y - (self.radius as i32 - 1),
            ),
            (self.radius - 1) * 2,
        )
        .into_styled(bezel_style)
        .draw(display)
        .map_err(|_| anyhow::anyhow!("Failed to draw bezel"))?;

        // Inner shadow (subtle)
        let shadow_style = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb565::new(0x08, 0x08, 0x08))
            .stroke_width(1)
            .build();
        Circle::new(
            Point::new(
                self.center.x - (self.radius as i32 - 5),
                self.center.y - (self.radius as i32 - 5),
            ),
            (self.radius - 5) * 2,
        )
        .into_styled(shadow_style)
        .draw(display)
        .map_err(|_| anyhow::anyhow!("Failed to draw inner shadow"))?;

        // Inner fill - pure black background like authentic Nest
        let bg_color = Rgb565::BLACK;
        Circle::new(
            Point::new(
                self.center.x - (self.radius as i32 - 14),
                self.center.y - (self.radius as i32 - 14),
            ),
            (self.radius - 14) * 2,
        )
        .into_styled(PrimitiveStyle::with_fill(bg_color))
        .draw(display)
        .map_err(|_| anyhow::anyhow!("Failed to draw inner fill"))?;

        // Subtle glass reflection effect (top-left to bottom-right)
        Arc::new(
            self.center,
            diameter - 20,
            Angle::from_degrees(-140.0),
            Angle::from_degrees(80.0),
        )
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_color(Rgb565::new(0x0C, 0x10, 0x0C))
                .stroke_width(2)
                .build(),
        )
        .draw(display)
        .map_err(|_| anyhow::anyhow!("Failed to draw glass reflection"))?;

        // Target ring (if available)
        if let Some(target) = self.target_temp {
            let fraction = self.temp_fraction(match self.unit {
                TemperatureUnit::Celsius => target,
                TemperatureUnit::Fahrenheit => (target - 32.0) * 5.0 / 9.0,
            });
            let target_arc = Arc::new(
                self.center,
                diameter,
                Angle::from_degrees(-135.0),
                Angle::from_degrees(fraction * 270.0),
            );
            let stroke = match self.hvac_mode {
                HvacDisplayMode::Heat => Rgb565::new(0x1F, 0x19, 0x00), // MAX VIBRANT orange
                HvacDisplayMode::Cool => Rgb565::new(0x00, 0x3F, 0x1F), // MAX VIBRANT cyan
                HvacDisplayMode::HeatCool => Rgb565::CSS_WHITE,
                HvacDisplayMode::Eco => Rgb565::new(0x00, 0x3F, 0x10), // Bright green
                HvacDisplayMode::Off => Rgb565::CSS_DARK_GRAY,
            };

            // Draw target arc with prominent gradient effect
            // Outer shadow/glow (darkest)
            let shadow_style = PrimitiveStyleBuilder::new()
                .stroke_color(Rgb565::new(stroke.r() / 3, stroke.g() / 3, stroke.b() / 3))
                .stroke_width(16)
                .build();
            Arc::new(
                self.center,
                diameter + 3,
                Angle::from_degrees(-135.0),
                Angle::from_degrees(fraction * 270.0),
            )
            .into_styled(shadow_style)
            .draw(display)
            .map_err(|_| anyhow::anyhow!("Failed to draw arc shadow"))?;

            // Mid layer (transition)
            let mid_style = PrimitiveStyleBuilder::new()
                .stroke_color(Rgb565::new(
                    stroke.r() * 2 / 3,
                    stroke.g() * 2 / 3,
                    stroke.b() * 2 / 3,
                ))
                .stroke_width(12)
                .build();
            Arc::new(
                self.center,
                diameter + 1,
                Angle::from_degrees(-135.0),
                Angle::from_degrees(fraction * 270.0),
            )
            .into_styled(mid_style)
            .draw(display)
            .map_err(|_| anyhow::anyhow!("Failed to draw arc mid layer"))?;

            // Main arc (full brightness)
            let target_style = PrimitiveStyleBuilder::new()
                .stroke_color(stroke)
                .stroke_width(8)
                .build();
            target_arc
                .into_styled(target_style)
                .draw(display)
                .map_err(|_| anyhow::anyhow!("Failed to draw target arc"))?;
        }

        // Current temperature marker
        let current_fraction = self.temp_fraction(match self.unit {
            TemperatureUnit::Celsius => self.current_temp,
            TemperatureUnit::Fahrenheit => (self.current_temp - 32.0) * 5.0 / 9.0,
        });
        let marker_angle = Angle::from_degrees(-135.0 + current_fraction * 270.0);
        let marker_distance = self.radius as f32 - 6.0;
        let marker_radians = marker_angle.to_radians();
        let marker_point = Point::new(
            self.center.x + (marker_distance * marker_radians.cos()).round() as i32,
            self.center.y + (marker_distance * marker_radians.sin()).round() as i32,
        );
        // Draw current temperature marker with gradient glow effect
        // Outer glow
        Circle::new(Point::new(marker_point.x - 7, marker_point.y - 7), 14)
            .into_styled(PrimitiveStyle::with_fill(Rgb565::new(0x0C, 0x00, 0x00)))
            .draw(display)
            .map_err(|_| anyhow::anyhow!("Failed to draw marker glow"))?;

        // Main marker
        Circle::new(Point::new(marker_point.x - 5, marker_point.y - 5), 10)
            .into_styled(PrimitiveStyle::with_fill(Rgb565::new(0x1F, 0x00, 0x00)))
            .draw(display)
            .map_err(|_| anyhow::anyhow!("Failed to draw marker"))?;

        // Inner highlight
        Circle::new(Point::new(marker_point.x - 2, marker_point.y - 2), 4)
            .into_styled(PrimitiveStyle::with_fill(Rgb565::new(0x1F, 0x0C, 0x00)))
            .draw(display)
            .map_err(|_| anyhow::anyhow!("Failed to draw marker highlight"))?;

        // Draw current temperature (LARGE, bold, centered) - main focus!
        let temp_value = format!("{:.0}", self.current_temp);
        let text_color = if self.is_heating {
            Rgb565::new(0x1F, 0x19, 0x00) // VIBRANT orange for heating (max brightness in RGB565)
        } else if self.is_cooling {
            Rgb565::new(0x00, 0x3F, 0x1F) // VIBRANT cyan for cooling
        } else {
            Rgb565::CSS_WHITE
        };
        let text_style = MonoTextStyle::new(&FONT_10X20, text_color);

        // Draw MASSIVE CLEAR temperature number - the absolute centerpiece
        // Use 3x3 grid for maximum size and clarity
        for dy in -1..=1 {
            for dx in -1..=1 {
                for repeat in 0..4 {
                    Text::with_alignment(
                        &temp_value,
                        Point::new(self.center.x + dx + repeat, self.center.y + dy + repeat),
                        text_style,
                        Alignment::Center,
                    )
                    .draw(display)
                    .ok();
                }
            }
        }

        // Draw degree symbol - large and clear
        for dy in -1..=1 {
            for dx in -1..=1 {
                for repeat in 0..3 {
                    Text::with_alignment(
                        "°",
                        Point::new(
                            self.center.x + 30 + dx + repeat,
                            self.center.y - 5 + dy + repeat,
                        ),
                        text_style,
                        Alignment::Center,
                    )
                    .draw(display)
                    .ok();
                }
            }
        }

        // Draw subtle Nest branding at very top
        let brand_style = MonoTextStyle::new(&FONT_10X20, Rgb565::new(0x08, 0x10, 0x08));
        Text::with_alignment(
            "Nest",
            Point::new(self.center.x, self.center.y - self.radius as i32 + 25),
            brand_style,
            Alignment::Center,
        )
        .draw(display)
        .map_err(|_| anyhow::anyhow!("Failed to draw brand label"))?;

        // Draw CURRENT label above main temperature
        let label_style = MonoTextStyle::new(&FONT_6X10, Rgb565::new(0x10, 0x10, 0x10));
        Text::with_alignment(
            "CURRENT",
            Point::new(self.center.x, self.center.y - 50),
            label_style,
            Alignment::Center,
        )
        .draw(display)
        .map_err(|_| anyhow::anyhow!("Failed to draw current label"))?;

        // Draw current temperature (smaller, above main temp)
        let current_temp_color = Rgb565::new(0x14, 0x14, 0x14); // Soft gray
        let current_style = MonoTextStyle::new(&FONT_6X10, current_temp_color);
        let current_text = format!("{}°", self.current_temp.round() as i32);
        Text::with_alignment(
            &current_text,
            Point::new(self.center.x, self.center.y - 80),
            current_style,
            Alignment::Center,
        )
        .draw(display)
        .map_err(|_| anyhow::anyhow!("Failed to draw current temp"))?;

        // Draw target temperature info below main display
        if let Some(target) = self.target_temp {
            let target_text = format!("SET TO {:.0}°", target);
            let target_color = Rgb565::CSS_GRAY;
            let target_style = MonoTextStyle::new(&FONT_10X20, target_color);
            Text::with_alignment(
                &target_text,
                Point::new(self.center.x, self.center.y + 70),
                target_style,
                Alignment::Center,
            )
            .draw(display)
            .map_err(|_| anyhow::anyhow!("Failed to draw target temp"))?;
        }

        // Draw current temperature (smaller, above main temp)
        let current_temp_color = Rgb565::new(0x14, 0x14, 0x14); // Soft gray
        let current_style = MonoTextStyle::new(&FONT_6X10, current_temp_color);
        let current_text = format!("{}°", self.current_temp.round() as i32);
        Text::with_alignment(
            &current_text,
            Point::new(self.center.x, self.center.y - 80),
            current_style,
            Alignment::Center,
        )
        .draw(display)
        .map_err(|_| anyhow::anyhow!("Failed to draw current temp"))?;

        // Draw status at bottom - BOLD and PROMINENT when active
        let status_label = if self.is_heating {
            "HEATING"
        } else if self.is_cooling {
            "COOLING"
        } else {
            ""
        };

        if !status_label.is_empty() {
            let status_color = if self.is_heating {
                Rgb565::new(0x1F, 0x19, 0x00) // MAX orange
            } else {
                Rgb565::new(0x00, 0x3F, 0x1F) // MAX cyan
            };
            let status_style = MonoTextStyle::new(&FONT_10X20, status_color);

            // Draw bold status text
            for dy in 0..=2 {
                for dx in 0..=2 {
                    Text::with_alignment(
                        status_label,
                        Point::new(
                            self.center.x + dx,
                            self.center.y + self.radius as i32 - 30 + dy,
                        ),
                        status_style,
                        Alignment::Center,
                    )
                    .draw(display)
                    .ok();
                }
            }
        }

        // Draw mode indicator (icon-style text)
        let mode_icon = match self.hvac_mode {
            HvacDisplayMode::Heat => "🔥",
            HvacDisplayMode::Cool => "❄️",
            HvacDisplayMode::HeatCool => "🔄",
            HvacDisplayMode::Eco => "🍃",
            HvacDisplayMode::Off => "⭕",
        };
        let icon_style = MonoTextStyle::new(&FONT_6X10, Rgb565::new(0x10, 0x10, 0x10));
        Text::with_alignment(
            mode_icon,
            Point::new(self.center.x, self.center.y + self.radius as i32 - 45),
            icon_style,
            Alignment::Center,
        )
        .draw(display)
        .map_err(|_| anyhow::anyhow!("Failed to draw mode icon"))?;

        Ok(())
    }

    fn handle_event(&mut self, event: &Event) -> Result<EventResult> {
        match event {
            Event::TemperatureUpdate { celsius } => {
                self.current_temp = *celsius;
                Ok(EventResult::NeedsRedraw)
            }
            Event::HvacStateChanged {
                mode,
                is_heating,
                is_cooling,
            } => {
                self.hvac_mode = *mode;
                self.is_heating = *is_heating;
                self.is_cooling = *is_cooling;
                Ok(EventResult::NeedsRedraw)
            }
            _ => Ok(EventResult::NotHandled),
        }
    }

    fn update(&mut self, _dt: Duration) -> Result<()> {
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
