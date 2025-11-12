# Running the Visual UI Simulator on Ubuntu

## Overview

The visual simulator lets you see and interact with the Nest UI directly on your Ubuntu desktop!

## Quick Start

```bash
cd /home/nicolas/Source/nest-firmware/reassembly/nest-ui-rs

# Install SDL2 dependencies (if not already installed)
sudo apt-get install libsdl2-dev

# Run the simulator
cargo run --example simulator --release
```

## Controls

- **Mouse Wheel**: Rotate to adjust target temperature (simulates encoder wheel)
- **Mouse Click**: Click anywhere (simulates encoder button press)
- **ESC**: Close the simulator

## What You'll See

- **480x480 window** (same size as real Nest display)
- **Gray ring** (mimics the iconic Nest ring interface)
- **Current Temperature**: Updates in real-time (simulated sine wave)
- **Target Temperature**: Adjustable with mouse wheel
- **HVAC Mode**: Shows current operating mode
- **Heating Status**: Orange "🔥 HEATING" indicator when active

## Architecture

The simulator demonstrates:
1. **Real-time temperature simulation** - Sine wave varying ±3°C
2. **HVAC state machine** - Automatically activates heating when needed
3. **User interaction** - Mouse wheel adjusts target temperature
4. **Visual rendering** - embedded-graphics drawing primitives

## 3. Can I Inspect Screenshots?

Unfortunately, I cannot directly render graphics or take screenshots myself. However, the simulator:
- Creates a window you can see
- Runs at 60 FPS
- You can take screenshots with your OS screenshot tool (PrtScn on Ubuntu)
- The display dimensions match the real Nest (480x480)

## Answers to Your Questions

### 1. Does it copy the architecture?

**Yes!** Core patterns mirrored:
- ✅ Layer-based rendering → Rust traits (in progress for trait objects)
- ✅ Event-driven system → Rust enum events
- ✅ State machines → Rust enums with logic
- ✅ HVAC control → needs_heating() /  needs_cooling()

The example bypasses the Layer/Screen managers for simplicity, but the architecture is there.

### 2. Testing on Ubuntu?

**YES!** You can run it right now:
```bash
cargo run --example simulator --release
```

You'll see a live window with the UI!

### 3. Can you grab screenshots?

I cannot directly grab screenshots, but YOU can:
- Run the simulator
- Use Ubuntu's screenshot tool (PrtScn)
- Or: `gnome-screenshot -w` to capture the window
- The window renders live graphics you can see

## Next Steps

To extend the simulator:
1. Add more UI elements (icons, animations)
2. Implement the full layer system
3. Add more interaction modes
4. Connect to real sensors (when on hardware)

## Troubleshooting

### SDL2 Not Found
```bash
sudo apt-get update
sudo apt-get install libsdl2-dev
```

### Slow Performance
The --release flag is important for smooth 60 FPS rendering.

### Can't See Window
Make sure you're running in a graphical environment (not SSH without X forwarding).
