
mod parameters;
mod math;
mod input;
mod channel;
mod tui;
mod experiments;

use std::io;
use experiments::run_experiments;
use clap::{Args, Parser, Subcommand};
use parameters::Muller;
use tui::setup::setup_screen;

#[derive(Parser, Default)]
struct Cli {
    #[clap(subcommand)]
    mode: Mode,
}

#[derive(Subcommand, Default)]
enum Mode {
    #[clap(name = "experiments", about = "Run experiments")]
    Experiments(Params),

    #[clap(name = "tui", about = "Run TUI")]
    #[default] TUI
}

#[derive(Args, Clone)]
struct Params {
    #[arg(short = None, long = "m1", default_value = "10", help = "Muller iterations for the efficiency experiment")]
    muller1: Muller,
    #[arg(short = None, long = "m2", default_value = "5", help = "Muller iterations for the decoding time experiment")]
    muller2: Muller,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();
    match args.mode {
        Mode::Experiments(params) => {
            run_experiments(params.muller1, params.muller2);
        }
        Mode::TUI => {
            let mut terminal = cursive::default();
            terminal.add_global_callback('q', |term| term.quit());
        
            setup_screen(&mut terminal);
        
            terminal.run();
        }
    }

    Ok(())
}