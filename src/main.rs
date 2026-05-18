use cdsd::cli::Args;
use cdsd::run;
use clap::Parser;

fn main() {
    let args = Args::parse();
    run(&args);
}
