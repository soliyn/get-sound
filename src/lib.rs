use std::fs::File;
use std::path::Path;
use std::process;

use crate::cambridge_dictionary::{CambridgeDictionary, PronunciationRegion, WordInfo};
use crate::cli::Args;
use crate::csv_helper::get_words;

pub use crate::cambridge_dictionary::Language;

mod cambridge_dictionary;
pub mod cli;
mod csv_helper;
mod http_helper;

pub fn run(config: &Args) {
    let file = File::open(&config.input_path).unwrap_or_else(|err| {
        eprintln!("Error opening file '{:?}': {}", &config.input_path, err);
        process::exit(1);
    });
    let words = get_words(file, config.column_index);
    let dict = CambridgeDictionary::new();
    for word in words {
        let result = dict.get_word(config.language, &word);
        match result {
            Ok(Some(wi)) => {
                process_word(&wi, Path::new(""), &PronunciationRegion::Us);
            }
            Ok(None) => eprintln!("Could not parse '{}'", &word),
            Err(err) => eprintln!("Error getting '{}': {}", &word, err),
        }
    }
}

fn process_word(wi: &WordInfo, dir: &Path, preferred_pronunciation: &PronunciationRegion) {
    // download sounds to the folder
    let p = wi
        .pronunciations
        .iter()
        .find(|x| x.region == *preferred_pronunciation)
        .unwrap_or_else(|| &wi.pronunciations[0]);
    // add the word to the output csv
    todo!()
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn
// }
