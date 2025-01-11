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
pub mod widgets;

fn main() -> cushy::Result {
    let args = Args::parse();
    let mut app = PendingApp::new(TokioRuntime::default());

    app.on_startup(move |app| match args.cmd {
        Commands::Bar => start_bar(app).unwrap(),
        Commands::Menu => start_menu(app).unwrap(),
        Commands::Power => {
            "Hello world!".open(app).unwrap();
        }
    });

    // Ok(())
    app.run()
}
