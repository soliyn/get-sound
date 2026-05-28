use super::*;
use rstest::*;

#[rstest]
#[case(
    "stark",
    Some("stɑːk".to_string()),
    Some("/media/english/uk_pron/u/uks/uksta/ukstarg004.mp3".to_string()),
    Some("stɑːrk".to_string()),
    Some("/media/english/us_pron/s/sta/stark/stark.mp3".to_string()),
)]
#[case(
    "apple",
    Some("ˈæp.əl".to_string()),
    Some("/media/english/uk_pron/u/uka/ukapp/ukappen014.mp3".to_string()),
    Some("ˈæp.əl".to_string()),
    Some("/media/english/us_pron/a/app/apple/apple.mp3".to_string()),
)]
fn should_parse_word_html(
    #[case] word: &str,
    #[case] expected_ipa_uk: Option<String>,
    #[case] expected_audio_uk: Option<String>,
    #[case] expected_ipa_us: Option<String>,
    #[case] expected_audio_us: Option<String>,
) {
    use std::fs;

    let html_path = format!("tests/fixtures/{}_cambridge.html", word);
    let html = fs::read_to_string(&html_path)
        .unwrap_or_else(|err| panic!("Failed to read file at {}: {}", html_path, err));
    let word_info = parse_word_html(&html).unwrap();
    let word_info_expected = WordInfo {
        word: word.to_string(),
        pronunciations: vec![
            WordPronunciation {
                region: PronunciationRegion::Uk,
                ipa_transcription: expected_ipa_uk,
                audio_mpeg: expected_audio_uk,
            },
            WordPronunciation {
                region: PronunciationRegion::Us,
                ipa_transcription: expected_ipa_us,
                audio_mpeg: expected_audio_us,
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
