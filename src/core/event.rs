use std::time::Duration;

/// Events that can be sent to layers and screens
#[derive(Debug, Clone)]
pub enum Event {
    /// Encoder wheel rotated (delta: positive = clockwise, negative = counter-clockwise)
    EncoderRotate {
        delta: i32,
    },

    /// Encoder wheel pressed
    EncoderPress,

    /// Encoder wheel released
    EncoderRelease,

    /// Long press detected
    EncoderLongPress,

    /// Touch event (for touchscreen variants)
    Touch {
        x: u32,
        y: u32,
    },

    /// Motion detected (Farsight)
    MotionDetected,

    /// Temperature sensor reading
    TemperatureUpdate {
        celsius: f32,
    },

    /// Humidity sensor reading
    HumidityUpdate {
        percent: f32,
    },

    /// Timer tick
    Tick {
        elapsed: Duration,
    },

    /// Network status change
    NetworkStatusChanged {
        connected: bool,
    },

    /// HVAC state change
    HvacStateChanged {
        mode: HvacDisplayMode,
        is_heating: bool,
        is_cooling: bool,
    },

    /// Software update available
    UpdateAvailable {
        version: String,
    },

    /// Display fade requested
    DisplayFadeUp,
    DisplayFadeDown,

    /// Screen transition
    TransitionTo {
        screen_id: String,
    },

    /// Back button/gesture
    Back,

    /// Wake up display
    WakeUp {
        reason: WakeupReason,
    },

    /// Go to sleep
    Sleep,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WakeupReason {
    EncoderActivity,
    MotionDetected,
    ScheduleChange,
    TemperatureAlert,
    NetworkActivity,
    SystemTest,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HvacDisplayMode {
    Off,
    Heat,
    Cool,
    HeatCool,
    Eco,
}

/// Result of event handling
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventResult {
    /// Event was handled
    Handled,

    /// Event was not handled, pass to parent
    NotHandled,

    /// Request screen transition
    TransitionTo(String),

    /// Request back navigation
    GoBack,

    /// Exit application
    Exit,

    /// Refresh display
    NeedsRedraw,
}
