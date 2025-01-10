use cushy::{
    figures::{
        units::{Lp, UPx},
        Size,
    },
    kludgine::app::winit::{platform::wayland::Anchor, window::WindowLevel},
    styles::{
        components::{
            BaseLineHeight, BaseTextSize, CornerRadius, DefaultBackgroundColor, FontWeight,
        },
        FontFamilyList,
    },
    value::Dynamic,
    widget::MakeWidget,
    Application, Open,
};

use crate::theme::{BG_DEFAULT, CORNER_RADIUS, DEFAULT_FONT_WEIGHT, TEXT_FONT, TEXT_SIZE};

mod spotify;
mod time;

pub fn start_bar(app: &mut impl Application) -> cushy::Result {
    let monitors = (app.as_app().monitors()).unwrap();
    let mut monitor_size: Size<UPx> = monitors.available[0].size().into();
    monitor_size.height = UPx::new(40);
    let size = Dynamic::new(monitor_size);
    let mut window = (time::time_widget().pad())
        .and(spotify::spotify_controls().pad())
        .into_columns()
        .centered()
        .expand_horizontally()
        .height(Lp::points(30))
        .with(&BaseTextSize, TEXT_SIZE)
        .with(&BaseLineHeight, TEXT_SIZE)
        .with(&DefaultBackgroundColor, BG_DEFAULT)
        .with(&CornerRadius, CORNER_RADIUS)
        .with(&FontWeight, DEFAULT_FONT_WEIGHT)
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
