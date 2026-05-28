use anyhow::Result;
use std::fs::File;

use cdsd::{App, cli::Args};
use clap::Parser;
use simplelog::{CombinedLogger, ConfigBuilder, LevelFilter, WriteLogger};

fn main() -> Result<()> {
    let config = ConfigBuilder::new()
        .add_filter_allow_str("cdsd") // ← your crate name here
        .build();
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Trace,
        config,
        File::create("app.log").unwrap(),
    )])
    .unwrap();

    let args = Args::parse();

    App::run(&args)
}
