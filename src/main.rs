use clap::Parser;
use rust_jorney::{Args, run};

fn main() {
    let args: Args = Args::parse();

    let searchable = &args.searchable;
    let file_path = &args.filepath;
    let ignore_case = args.ignore_case;

    if let Err(e) = crate::run(args) {
        eprintln!("The Error ocured: {}", e);
        std::process::exit(1);
    }
}
