use bar::start_bar;
use cushy::{PendingApp, Run, TokioRuntime};

mod vibrancy;
mod theme;
mod bar;

fn main() -> cushy::Result {
    let mut app = PendingApp::new(TokioRuntime::default());
    start_bar(&mut app)?;

    app.run()
}