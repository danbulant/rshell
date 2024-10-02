use cushy::{Open, PendingApp, Run, TokioRuntime};

mod spotify;
mod vibrancy;

fn main() -> cushy::Result {
    let mut app = PendingApp::new(TokioRuntime::default());

    spotify::spotify_controls()
        .open(&mut app)?;

    app.run()
}