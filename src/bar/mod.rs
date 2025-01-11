use battery::battery;
use cushy::{
    figures::{
        units::{Lp, UPx},
        Size, Zero,
    },
    kludgine::app::winit::{error::EventLoopError, platform::wayland::Anchor, window::WindowLevel},
    styles::{
        components::{
            BaseLineHeight, BaseTextSize, CornerRadius, DefaultBackgroundColor, FontWeight,
            IntrinsicPadding,
        },
        Dimension, FontFamilyList,
    },
    value::Dynamic,
    widget::MakeWidget,
    Application, Open,
};
use hypr::{get_hyprland_state_sync, hyprland_active_title, hyprland_workspaces, init_callbacks};
use hyprland::data::{Client, Devices, Monitor, Workspace};

use crate::{
    theme::{BG_DEFAULT, CORNER_RADIUS, DEFAULT_FONT_WEIGHT, TEXT_FONT, TEXT_SIZE},
    widgets::WidgetExt,
};

mod battery;
mod hypr;
mod memory;
mod spotify;
mod time;

#[derive(Debug, PartialEq)]
pub struct HyprlandState {
    monitors: Vec<Monitor>,
    workspaces: Vec<Workspace>,
    devices: Devices,
    clients: Vec<Client>,
}

pub fn start_bar(app: &mut impl Application) -> cushy::Result {
    let state = Dynamic::new(get_hyprland_state_sync().map_err(|err| {
        dbg!(err);
        EventLoopError::ExitFailure(1)
    })?);

    init_callbacks(state.clone());

    let monitors = (app.as_app().monitors()).unwrap();
    let mut monitor_size: Size<UPx> = monitors.available[0].size().into();
    monitor_size.height = UPx::new(40);
    monitor_size.width = UPx::new((monitor_size.width.get() as f64 / 1.25) as _);
    let size = Dynamic::new(monitor_size);

    let left_part = (hyprland_workspaces(state.clone())
        .and(hyprland_active_title(state.clone()))
        .into_columns()
        .bar_pill()
        .and(memory::memory_widget().bar_pill()))
    .into_columns();

    let middle_part = time::time_widget()
        .bar_pill()
        .and(spotify::spotify_controls().bar_pill())
        .into_columns()
        .centered()
        .expand_horizontally();

    let right_part = battery().bar_pill();

    let mut window = left_part
        .and(middle_part)
        .and(right_part)
        .into_columns()
        .expand_horizontally()
        .width(monitor_size.width)
        .height(Lp::points(30))
        .with(&BaseTextSize, TEXT_SIZE)
        .with(&BaseLineHeight, TEXT_SIZE)
        .with(&DefaultBackgroundColor, BG_DEFAULT)
        .with(&CornerRadius, CORNER_RADIUS)
        .with(&FontWeight, DEFAULT_FONT_WEIGHT)
        .with(&IntrinsicPadding, Dimension::ZERO)
        .into_window()
        .inner_size(size.clone())
        .titled("rshell")
        .transparent()
        .app_name("rshell")
        .decorated(false)
        .resize_to_fit(false)
        .window_level(WindowLevel::AlwaysOnTop);

    let mut family = FontFamilyList::default();
    for font in TEXT_FONT.iter() {
        family.push(cushy::styles::FamilyOwned::Name((*font).into()));
    }

    window.sans_serif_font_family = family;

    window.open(app).map(|handle| {
        handle.execute(move |ctx| {
            // safe unwrap: we just created the window
            let winit = ctx.winit().unwrap();
            winit.set_exclusive_zone(40);
            winit.set_anchor(Anchor::LEFT | Anchor::TOP | Anchor::RIGHT);
        });
    })
}
