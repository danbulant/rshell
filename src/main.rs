use bar::start_bar;
use clap::Parser;
use cli::{Args, Commands};
use cushy::{Open, PendingApp, Run, TokioRuntime};
use menu::start_menu;

mod bar;
mod cli;
mod menu;
pub mod rt;
mod theme;
mod vibrancy;

fn main() -> cushy::Result {
    let args = Args::parse();
    let mut app = PendingApp::new(TokioRuntime::default());

    app.on_startup(move |app| match args.cmd {
        Commands::Bar => start_bar(app).unwrap(),
        Commands::Menu => start_menu(app).unwrap(),
        Commands::Power => {
            let win = "Hello world!".open(app).unwrap();
            // std::thread::spawn(move || {
            //     std::thread::sleep(std::time::Duration::from_secs(2));
            //     win.request_close();
            // });
        }
    });

    // Ok(())
    app.run()
}
