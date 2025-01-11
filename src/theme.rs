use cushy::{
    figures::units::Px,
    styles::{Color, Dimension, Edges, Weight},
};

pub const CORNER_RADIUS: Dimension = Dimension::Px(Px::new(8));
pub const TEXT_SIZE: Dimension = Dimension::Px(Px::new(13));
pub const WIDGET_PADDING: Edges<Dimension> = Edges {
    left: Dimension::Px(Px::new(9)),
    right: Dimension::Px(Px::new(9)),
    top: Dimension::Px(Px::new(4)),
    bottom: Dimension::Px(Px::new(4)),
};

pub const WORKSPACE_PADDING: Edges<Dimension> = Edges {
    left: Dimension::Px(Px::new(15)),
    right: Dimension::Px(Px::new(15)),
    top: Dimension::Px(Px::new(4)),
    bottom: Dimension::Px(Px::new(4)),
};
pub const WORKSPACE_CORNER_RADIUS: Dimension = Dimension::Px(Px::new(12));

pub const DEFAULT_FONT_WEIGHT: Weight = Weight::MEDIUM;

pub const BG_DEFAULT: Color = Color(0x191724FF);
pub const TEXT_SPOTIFY: Color = Color(0x1DB954FF);
pub const TEXT_CLOCK: Color = Color(0xF6C177FF);
pub const TEXT_CPU: Color = Color(0xff671fFF);
pub const TEXT_MEM: Color = Color(0xFFFFFFFF);
pub const TEXT_TEMP: Color = Color(0x97f993FF);
pub const TEXT_RED: Color = Color(0xD81E5BFF);

pub const TEXT_TOOL: Color = Color(0x4e9dc2ff);

pub const TEXT_AUDIO: Color = Color(0xF7E733FF);
pub const TEXT_AUDIO_MUTED: Color = Color(0xD81E5BFF);
pub const TEXT_BATTERY: Color = Color(0x6bb0d9FF);

pub const BG_WORKSPACE_ACTIVE: Color = Color(0x753a88ff);
pub const TEXT_WORKSPACE_ACTIVE: Color = Color(0xebbcbaff);

pub const TEXT_FONT: [&str; 5] = [
    "Inter",
    "Iosevka",
    "Iosevka NF",
    "Fira Code Retina",
    "FiraCode Nerd Font Mono",
];
