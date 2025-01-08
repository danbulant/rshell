use cushy::{
    figures::units::Px,
    styles::{Color, Dimension, Edges, Weight},
};

pub const CORNER_RADIUS: Dimension = Dimension::Px(Px::new(10));
pub const TEXT_SIZE: Dimension = Dimension::Px(Px::new(13));
pub const WIDGET_PADDING: Edges<Dimension> = Edges {
    left: Dimension::Px(Px::new(6)),
    right: Dimension::Px(Px::new(6)),
    top: Dimension::Px(Px::new(3)),
    bottom: Dimension::Px(Px::new(3)),
};

pub const DEFAULT_FONT_WEIGHT: Weight = Weight::BOLD;

pub const BG_DEFAULT: Color = Color(0x191724FF);
pub const TEXT_SPOTIFY: Color = Color(0x1DB954FF);
pub const TEXT_CLOCK: Color = Color(0xF6C177FF);
pub const TEXT_CPU: Color = Color(0xff671fFF);
pub const TEXT_MEM: Color = Color(0x1DB954FF);
pub const TEXT_TEMP: Color = Color(0x97f993FF);
