use std::io::{Read, Write};
use std::path::Path;
use std::{
    fs::{File, OpenOptions},
    process,
};

use anyhow::{self, Error, Result};

use crate::Language;
use crate::cambridge_dictionary::{Dictionary, WordPronunciation};
use crate::{
    cambridge_dictionary::{CambridgeDictionary, PronunciationRegion, WordInfo},
    cli::Args,
    csv_helper::get_words,
    http_helper::{Downloader, HttpHelper},
};

const DEFAULT_BASE_URL: &str = "https://dictionary.cambridge.org";
pub struct App {}

impl App {
    pub fn run(args: &Args) -> Result<()> {
        let input_file = File::open(&args.input_csv_file).unwrap_or_else(|err| {
            eprintln!("Error opening file '{:?}': {}", &args.input_csv_file, err);
            process::exit(1);
        });
        let output_csv_file = args
            .output_csv_file
            .clone()
            .unwrap_or_else(|| args.input_csv_file.with_file_name("output.csv"));
        let output_file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&output_csv_file)
            .unwrap();
        let dict = CambridgeDictionary::new(DEFAULT_BASE_URL.to_string());
        let http_helper = HttpHelper::new();

        App::run_internal(RunInternalParams {
            input_file,
            word_column_index: args.word_column_index,
            output_file,
            dict,
            language: args.language,
            http_helper,
            media_download_dir: &args.media_download_dir,
            preferred_pronunciation: args.preferred_pronunciation,
        });

        Ok(())
    }

    fn run_internal<R, RW, D, DL, P>(mut params: RunInternalParams<R, RW, D, DL, P>)
    where
        R: Read,
        RW: Read + Write,
        D: Dictionary,
        DL: Downloader,
        P: AsRef<Path>,
    {
        // fn run_internal(
        //     mut params: RunInternalParams<
        //         impl Read,
        //         impl Read + Write,
        //         impl Dictionary,
        //         impl Downloader,
        //         impl AsRef<Path>,
        //     >,
        // ) {
        let processed_words = get_words(&mut params.output_file, 0);
        let words = get_words(params.input_file, params.word_column_index);
        let words = words.into_iter().filter(|w| !processed_words.contains(w));

        for word in words {
            print!("getting \"{}\"", word);
            let result = params.dict.get_word(params.language, &word);
            match result {
                Ok(Some(wi)) => {
                    match process_word(
                        &wi,
                        &params.preferred_pronunciation,
                        &params.media_download_dir,
                        &mut params.output_file,
                        &params.http_helper,
                    ) {
                        Ok(_) => print!(" OK"),
                        Err(err) => {
                            print!(" ERROR: {}", err);
                            eprintln!("Could not process word '{}'", word);
                        }
                    }
                }
                Ok(None) => eprintln!("Could not parse '{}'", word),
                Err(err) => eprintln!("Error getting '{}': {}", word, err),
            }
            println!();
        }
    }
}

struct RunInternalParams<R, RW, D, DL, P>
where
    R: Read,
    RW: Read + Write,
    D: Dictionary,
    DL: Downloader,
    P: AsRef<Path>,
{
    input_file: R,
    word_column_index: usize,
    output_file: RW,
    dict: D,
    language: Language,
    http_helper: DL,
    media_download_dir: P,
    preferred_pronunciation: PronunciationRegion,
}

fn process_word<DL, W, P>(
    wi: &WordInfo,
    preferred_pronunciation: &PronunciationRegion,
    media_download_dir: P,
    output_file: &mut W,
    http_helper: &DL,
) -> Result<(), Error>
where
    DL: Downloader,
    W: Write,
    P: AsRef<Path>,
{
    let op = wi
        .pronunciations
        .iter()
        .find(|x| x.region == *preferred_pronunciation)
        .or_else(|| wi.pronunciations.first());

    let Some(p) = op else {
        writeln!(output_file, "{}\t\t", wi.word)?;
        return Err(anyhow::anyhow!("No pronunciation found"));
    };
    let ipa_transcription = p.ipa_transcription.clone().unwrap_or_default();
    let Some(audio_mpeg) = &p.audio_mpeg else {
        writeln!(output_file, "{}\t{}\t", wi.word, ipa_transcription)?;
        return Err(anyhow::anyhow!("No audio file found"));
    };

    let audio_file_name = get_audio_file_name(&wi.word, &p.region);
    let mut sound_file = File::create(media_download_dir.as_ref().join(&audio_file_name))?;
    http_helper.download_file(audio_mpeg, &mut sound_file)?;

    writeln!(
        output_file,
        "{}\t{}\t[sound:{}]",
        wi.word, ipa_transcription, audio_file_name
    )?;

    Ok(())
}

fn get_audio_file_name(word: &str, region: &PronunciationRegion) -> String {
    format!("{}_{}.mp3", word, region)
}

#[cfg(test)]
mod tests;
