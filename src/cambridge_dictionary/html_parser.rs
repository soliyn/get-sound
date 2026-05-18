use super::*;
use scraper::Selector;
use scraper::{ElementRef, Html};

pub fn parse_word_html(html: &str) -> Option<WordInfo> {
    let document = Html::parse_document(html);

    let word_entry_selector = Selector::parse("div.pos-header.dpos-h").unwrap();
    let word_entry = document.select(&word_entry_selector).nth(0)?;

    let word_selector = Selector::parse("span.hw.dhw").unwrap();
    let word = word_entry.select(&word_selector).nth(0)?;

    let pronunciation_selector = Selector::parse("span.dpron-i").unwrap();
    let pronunciations = word_entry.select(&pronunciation_selector);

    Some(WordInfo {
        word: word.text().collect::<String>(),
        pronunciations: pronunciations
            .filter_map(|p| parse_pronunciation(&p))
            .collect(),
    })
}

fn parse_pronunciation(pronunciation_html: &ElementRef) -> Option<WordPronunciation> {
    Some(WordPronunciation {
        region: parse_region(pronunciation_html)?,
        ipa_transcription: parse_ipa_transcription(pronunciation_html),
        audio_mpeg: parse_audio(pronunciation_html),
    })
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
            _ => None,
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
mod tests;
