use std::path::PathBuf;

use clap::Parser;

use crate::Language;

#[derive(Parser, Debug)]
#[command(version, about = "Pronunciation downloader from Cambridge dictionary")]
pub struct Args {
    #[arg(
            short,
            long,
            value_name = "INPUT_FILE",
            value_parser = validate_csv_path
        )]
    pub input_path: PathBuf,

    #[arg(short, long, value_name = "OUTPUT_FILE")]
    pub output_path: Option<PathBuf>,

    pub media_dir: Option<PathBuf>,

    #[arg(short, long)]
    pub column_index: usize,

    #[arg(short, long, value_enum)]
    pub language: Language,
}

fn validate_csv_path(path_str: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(path_str);

    if !path.exists() {
        return Err(format!("The file '{}' does not exist.", path_str));
    }

    if !path.is_file() {
        return Err(format!("'{}' is not a file.", path_str));
    }

    Ok(path)
}
