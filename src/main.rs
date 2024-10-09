use cushy::{styles::{components::DefaultBackgroundColor, Color}, widget::MakeWidget, Open, PendingApp, Run, TokioRuntime};

mod spotify;
mod vibrancy;

fn main() -> cushy::Result {
    let mut app = PendingApp::new(TokioRuntime::default());
    

    spotify::spotify_controls()
        .centered()
        .with(&DefaultBackgroundColor, Color::CLEAR_WHITE)
        .into_window()
        .transparent()
        .app_name("rshell")
        .open(&mut app)?;

    app.run()
}