//! Module defining the constants used for aesthetic purposes (colors, borders...)

use iced::font::{Family, Stretch, Style, Weight};
use iced::{Color, Font};
use std::time::Duration;

use crate::gui::sniffer::{FONT_FAMILY_NAME, ICON_FONT_FAMILY_NAME};

// main font
pub const SARASA_MONO_BYTES: &[u8] =
    include_bytes!("../../../resources/fonts/subset/sarasa-mono-sc-regular.subset.ttf");
pub const SARASA_MONO: Font = Font {
    family: Family::Name(FONT_FAMILY_NAME),
    weight: Weight::Normal,
    stretch: Stretch::Normal,
    style: Style::Normal,
};

//font to display icons
pub const ICONS_BYTES: &[u8] = include_bytes!("../../../resources/fonts/subset/icons.ttf");
pub const ICONS: Font = Font::with_name(ICON_FONT_FAMILY_NAME);

// font sizes - refined for better visual hierarchy
pub const FONT_SIZE_FOOTER: f32 = 13.5;
pub const FONT_SIZE_BODY: f32 = 15.5;
pub const FONT_SIZE_SUBTITLE: f32 = 17.0;
pub const FONT_SIZE_TITLE: f32 = 20.5;

// border styles - modernized with softer corners
pub const BORDER_WIDTH: f32 = 1.5;
pub const CHARTS_LINE_BORDER: u32 = 1;
pub const BORDER_ROUNDED_RADIUS: f32 = 20.0;
pub const BORDER_BUTTON_RADIUS: f32 = 180.0;

// red colors for alerts - more vibrant
pub const RED_ALERT_COLOR_NIGHTLY: Color = Color {
    r: 1.0,
    g: 0.35,
    b: 0.35,
    a: 1.0,
};
pub const RED_ALERT_COLOR_DAILY: Color = Color {
    r: 0.85,
    g: 0.15,
    b: 0.15,
    a: 1.0,
};

// delays
pub const TOOLTIP_DELAY: Duration = Duration::from_millis(250);
