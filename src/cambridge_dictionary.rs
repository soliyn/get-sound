use crate::http_helper;
use clap::ValueEnum;
use html_parser::parse_word_html;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

mod html_parser;

#[derive(Debug, PartialEq, Eq)]
pub struct WordInfo {
    pub word: String,
    pub pronunciations: Vec<WordPronunciation>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PronunciationRegion {
    Uk,
    Us,
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
}

impl CambridgeDictionary {
    pub fn new() -> Self {
        Self {
            base_url: "https://dictionary.cambridge.org".to_string(), // /{language}/{word}
        }
    }
    pub fn get_word(
        &self,
        language: Language,
        word: &str,
    ) -> Result<Option<WordInfo>, Box<dyn Error>> {
        let word_html = http_helper::download_html(&format!(
            "{}/dictionary/{}/{}",
            self.base_url, language, word
        ))?;

        Ok(parse_word_html(&word_html).map(|wi| wi.add_base_url_to_audios(&self.base_url)))
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_base_url_to_audios_should_add_base_url_prefix() {
        let cd = CambridgeDictionary::new();
        const AUDIO_URL: &str = "/media/english/uk_pron/u/uks/uksta/ukstarg004.mp3";
        let mut wi = Some(WordInfo {
            word: "word".to_string(),
            pronunciations: vec![WordPronunciation {
                audio_mpeg: Some(AUDIO_URL.to_string()),
                region: PronunciationRegion::Us,
                ipa_transcription: None,
            }],
        });
        wi = wi.map(|wi| wi.add_base_url_to_audios(&cd.base_url));

        let expected_url = format!("{}{}", cd.base_url, AUDIO_URL);

        let actual_audio: Option<&String> = wi
            .as_ref()
            .and_then(|w| w.pronunciations.first())
            .and_then(|p| p.audio_mpeg.as_ref());

        assert_eq!(actual_audio, Some(&expected_url));
    }
}
