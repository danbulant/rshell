use cushy::{
    figures::{units::Px, Point},
    kludgine::app::winit::{platform::wayland::Anchor, window::WindowLevel},
    value::{Destination, Dynamic, Source},
    widget::MakeWidget,
    Application, Open,
};

mod spotify;

pub fn start_bar(app: &mut impl Application) -> cushy::Result {
    let pos = Dynamic::default();
    let mut window = spotify::spotify_controls()
        .pad()
        .centered()
        .expand_horizontally()
        .into_window()
        .transparent()
        .app_name("rshell")
        .decorated(false)
        .outer_position(pos.clone(), false)
        .window_level(WindowLevel::AlwaysOnTop);

    window
        .sans_serif_font_family
        .push(cushy::styles::FamilyOwned::Name("Iosevka NF".into()));

    window.open(app).map(|handle| {
        handle.execute(|ctx| {
            // safe unwrap: we just created the window
            let winit = ctx.winit().unwrap();
            winit.set_exclusive_zone(40);
            winit.set_anchor(Anchor::LEFT | Anchor::TOP | Anchor::RIGHT);
        });
    })
}
