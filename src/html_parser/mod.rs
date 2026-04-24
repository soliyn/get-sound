use scraper::{ElementRef, Html};
use scraper::Selector;

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

pub fn parse_word_html(html : &str) -> Option<WordInfo> {
    let document = Html::parse_document(html);

    let word_entry_selector = Selector::parse("div.pos-header.dpos-h").unwrap();
    let word_entry = document.select(&word_entry_selector).nth(0)?;

    let word_selector = Selector::parse("span.hw.dhw").unwrap();
    let word = word_entry.select(&word_selector).nth(0)?;

    let pronunciation_selector = Selector::parse("span.dpron-i").unwrap();
    let pronunciations = word_entry.select(&pronunciation_selector);
    
    Some(WordInfo {
        word: word.text().collect::<String>(),
        pronunciations: pronunciations.filter_map(|p| parse_pronunciation(&p)).collect(),
    })
}

fn parse_pronunciation(pronunciation_html: &ElementRef) -> Option<WordPronunciation> {
    Some(
        WordPronunciation {
            region: parse_region(pronunciation_html)?,
            ipa_transcription: parse_ipa_transcription(pronunciation_html),
            audio_mpeg: parse_audio(pronunciation_html),
        }
    )
}

fn parse_audio(pronunciation_html: &ElementRef) -> Option<String> {
    // <source type="audio/mpeg" src="...mp3">
    // <source type="audio/ogg"  src="...ogg">
    let source_sel = Selector::parse(r#"source[type="audio/mpeg"]"#).unwrap();
    pronunciation_html
        .select(&source_sel)
        .next()
        .map(|el| el.attr("src").unwrap().to_string())
}

fn parse_region(pronunciation_html: &ElementRef) -> Option<PronunciationRegion> {
    let region_sel = Selector::parse("span.region.dreg").unwrap();
    pronunciation_html
        .select(&region_sel)
        .next()
        .map(|el| el.text().collect::<String>().trim().to_lowercase())
        .map(|text| match text.as_str() {
            "us" => Some(PronunciationRegion::Us),
            "uk" => Some(PronunciationRegion::Uk),
            _ => None
        })
        .flatten()
}

fn parse_ipa_transcription(pronunciation_html: &ElementRef) -> Option<String> {
    // <span class="ipa dipa lpr-2 lpl-1">stɑːk</span>
    let ipa_sel = Selector::parse("span.ipa.dipa").unwrap();
    pronunciation_html
        .select(&ipa_sel)
        .next()
        .map(|el| el.text().collect::<String>().trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_word_html() {
        let html = include_str!("../../tests/fixtures/stark_cambridge.html");
        let word_info = parse_word_html(html).unwrap();
        let word_info_expected = WordInfo {
            word: "stark".to_string(),
            pronunciations: vec![
                WordPronunciation {
                    region: PronunciationRegion::Uk,
                    ipa_transcription: Some("stɑːk".to_string()),
                    audio_mpeg: Some("/media/english/uk_pron/u/uks/uksta/ukstarg004.mp3".to_string()),
                },
                WordPronunciation {
                    region: PronunciationRegion::Us,
                    ipa_transcription: Some("stɑːrk".to_string()),
                    audio_mpeg: Some("/media/english/us_pron/s/sta/stark/stark.mp3".to_string()),
                },
            ]
        };

        assert_eq!(word_info, word_info_expected);
    }

    const FRAGMENT: &str = r#"<span class="uk dpron-i ">
        <span class="region dreg">uk</span>
        <span class="daud">
            <audio class="hdn" preload="none" id="audio1" controlslist="nodownload">
                <source type="audio/mpeg" src="/media/english/uk_pron/u/uks/uksta/ukstarg004.mp3">
                <source type="audio/ogg"  src="/media/english/uk_pron_ogg/u/uks/uksta/ukstarg004.ogg">
            </audio>
        </span>
        <span class="pron dpron">/<span class="ipa dipa lpr-2 lpl-1">stɑːk</span>/</span>
    </span>"#;
    #[test]
    fn test_parse_pronunciation() {
        let document = Html::parse_fragment(FRAGMENT);
        let sel = Selector::parse("span.dpron-i").unwrap();
        let root = document.select(&sel).next().unwrap();

        let result = parse_pronunciation(&root);

        assert_eq!(result, Some(WordPronunciation {
            region: PronunciationRegion::Uk,
            ipa_transcription: Some("stɑːk".to_string()),
            audio_mpeg: Some("/media/english/uk_pron/u/uks/uksta/ukstarg004.mp3".to_string()),
        }));
    }
}