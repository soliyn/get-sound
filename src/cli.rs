use std::path::PathBuf;

use clap::Parser;

use crate::{Language, cambridge_dictionary::PronunciationRegion};

const DEFAULT_MEDIA_DIR: &str = "c:\\temp\\aaa\\";

#[derive(Parser, Debug)]
#[command(version, about = "Pronunciation downloader from Cambridge dictionary")]
pub struct Args {
    #[arg(
            short,
            long,
            value_name = "INPUT_FILE",
            value_parser = validate_csv_path
        )]
    pub input_csv_file: PathBuf,

    #[arg(short, long, value_name = "OUTPUT_FILE")]
    pub output_csv_file: Option<PathBuf>,

    #[arg(default_value = DEFAULT_MEDIA_DIR, value_name = "MEDIA_DIR")]
    pub media_download_dir: PathBuf,

    #[arg(short, long, default_value_t = 0)]
    pub word_column_index: usize,

    #[arg(short, long, value_enum, default_value_t = Language::English)]
    pub language: Language,

    #[arg(short, long, value_enum, default_value_t = PronunciationRegion::Us)]
    pub preferred_pronunciation: PronunciationRegion,
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
