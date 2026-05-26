use clap::Parser;
use rust_jorney::{Args, run};

fn main() {
    let args: Args = Args::parse();

    if let Err(e) = crate::run(args) {
        eprintln!("The Error ocured: {}", e);
        std::process::exit(1);
    }
}
