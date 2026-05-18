use super::*;

#[test]
fn should_parse_word_html() {
    let html = include_str!("../../../tests/fixtures/stark_cambridge.html");
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
        ],
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

    assert_eq!(
        result,
        Some(WordPronunciation {
            region: PronunciationRegion::Uk,
            ipa_transcription: Some("stɑːk".to_string()),
            audio_mpeg: Some("/media/english/uk_pron/u/uks/uksta/ukstarg004.mp3".to_string()),
        })
    );
}
