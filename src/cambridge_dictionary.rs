use crate::http_helper::{Downloader, HttpHelper};
use anyhow::Result;
use clap::ValueEnum;
use html_parser::parse_word_html;
use std::fmt::Formatter;
use std::fmt::{self, Display};

mod html_parser;

#[derive(Debug, PartialEq, Eq)]
pub struct WordInfo {
    pub word: String,
    pub pronunciations: Vec<WordPronunciation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum PronunciationRegion {
    Uk,
    Us,
}

impl Display for PronunciationRegion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let str_val = match self {
            PronunciationRegion::Uk => "uk",
            PronunciationRegion::Us => "us",
        };
        write!(f, "{}", str_val)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct WordPronunciation {
    pub region: PronunciationRegion,
    pub ipa_transcription: Option<String>,
    pub audio_mpeg: Option<String>,
}

impl WordInfo {
    pub fn add_base_url_to_audios(mut self, base_url: &str) -> Self {
        for p in &mut self.pronunciations {
            if let Some(audio) = &mut p.audio_mpeg {
                audio.insert_str(0, base_url);
            }
        }
        self
    }

    pub fn get_preferred_pronunciation_or_first(
        &self,
        preferred_pronunciation: &PronunciationRegion,
    ) -> Option<&WordPronunciation> {
        self.pronunciations
            .iter()
            .find(|x| &x.region == preferred_pronunciation)
            .or_else(|| self.pronunciations.first())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Language {
    English,
    German,
    French,
}
impl fmt::Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let str_val = match self {
            Language::English => "english",
            Language::German => "german",
            Language::French => "french",
        };
        write!(f, "{}", str_val)
    }
}

pub struct CambridgeDictionary {
    base_url: String,
    http_helper: HttpHelper,
}

impl CambridgeDictionary {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            http_helper: HttpHelper::new(),
        }
    }
}

pub trait Dictionary {
    fn get_word(&self, language: Language, word: &str) -> Result<Option<WordInfo>>;
}

impl Dictionary for CambridgeDictionary {
    fn get_word(&self, language: Language, word: &str) -> Result<Option<WordInfo>> {
        let word_html = self.http_helper.download_html(&format!(
            "{}/dictionary/{}/{}", // /{language}/{word}
            self.base_url, language, word
        ))?;

        Ok(parse_word_html(&word_html).map(|wi| wi.add_base_url_to_audios(&self.base_url)))
    }
}

#[cfg(test)]
mod word_info_tests;
