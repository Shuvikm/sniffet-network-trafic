#![allow(clippy::unreadable_literal)]

//! Neon Pulse theme
//! A modern, cyberpunk-inspired theme with vibrant accents and glassmorphism effects

use iced::color;

use crate::gui::styles::types::palette::Palette;
use crate::gui::styles::types::palette_extension::PaletteExtension;

/// Neon Pulse Dark - Deep dark with electric cyan and neon pink
pub static NEON_PULSE_DARK_PALETTE: std::sync::LazyLock<Palette> =
    std::sync::LazyLock::new(|| Palette {
        primary: color!(0x0d1117),      // Deep dark blue-black
        secondary: color!(0x00d4ff),    // Electric cyan
        outgoing: color!(0xff6ec7),     // Neon pink
        starred: color!(0xffd700, 0.8), // Gold with transparency
        text_headers: color!(0x0d1117), // Dark text for headers
        text_body: color!(0xe6edf3),    // Soft white body text
    });

pub static NEON_PULSE_DARK_PALETTE_EXTENSION: std::sync::LazyLock<PaletteExtension> =
    std::sync::LazyLock::new(|| NEON_PULSE_DARK_PALETTE.generate_palette_extension());

/// Neon Pulse Light - Clean white with blue and magenta accents
pub static NEON_PULSE_LIGHT_PALETTE: std::sync::LazyLock<Palette> =
    std::sync::LazyLock::new(|| Palette {
        primary: color!(0xf0f6fc),      // Clean off-white
        secondary: color!(0x0969da),    // Vivid blue
        outgoing: color!(0xbf3989),     // Magenta
        starred: color!(0xf7b955),      // Warm gold
        text_headers: color!(0xf0f6fc), // White headers
        text_body: color!(0x1f2328),    // Dark body text
    });

pub static NEON_PULSE_LIGHT_PALETTE_EXTENSION: std::sync::LazyLock<PaletteExtension> =
    std::sync::LazyLock::new(|| NEON_PULSE_LIGHT_PALETTE.generate_palette_extension());
