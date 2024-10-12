use bar::start_bar;
use clap::Parser;
use cli::{Args, Commands};
use cushy::{PendingApp, Run, TokioRuntime};
use menu::start_menu;

mod vibrancy;
mod theme;
mod bar;
mod cli;
mod menu;

fn main() -> cushy::Result {
    let args = Args::parse();
    let mut app = PendingApp::new(TokioRuntime::default());

    match args.cmd {
        Commands::Bar => start_bar(&mut app)?,
        Commands::Menu => start_menu(&mut app)?,
        Commands::Power => todo!(),
    }

    app.run()
}