use cushy::{kludgine::app::winit::window::WindowLevel, widget::MakeWidget, Application, Open};

pub fn start_menu(app: &mut impl Application) -> cushy::Result {
    let mut window = "Hello, World!"
        .pad()
        .into_window()
        .transparent()
        .app_name("rshell")
        .decorated(false)
        // .resizable(false)
        .window_level(WindowLevel::AlwaysOnTop);

    window.sans_serif_font_family.push(cushy::styles::FamilyOwned::Name("Iosevka NF".into()));

    window
        .open(app).map(|_| ())
}