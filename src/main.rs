use clap::Parser;
use config::Config;
use ui::app::AppInit;

mod action;
mod config;
mod message;
mod ui;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "")]
    input: String,
}

fn main() {
    let arg = Args::parse();

    let config = Config::load();

    ui::run_app(AppInit {
        input: arg.input,
        actions: config.actions,
    });
}
