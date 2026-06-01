use rstest::rstest;
use std::assert_matches;

use super::*;

#[rstest]
#[case(PronunciationRegion::Uk)]
#[case(PronunciationRegion::Us)]
fn should_return_preferred_pronunciation(#[case] preferred_pronunciation: PronunciationRegion) {
    let word_info = WordInfo {
        word: "apple".to_string(),
        pronunciations: vec![
            WordPronunciation {
                region: PronunciationRegion::Uk,
                ipa_transcription: None,
                audio_mpeg: None,
            },
            WordPronunciation {
                region: PronunciationRegion::Us,
                ipa_transcription: None,
                audio_mpeg: None,
            },
        ],
    };

    let result = word_info.get_preferred_pronunciation_or_first(&preferred_pronunciation);

    assert_matches!(result, Some(WordPronunciation { region, .. }) if region == &preferred_pronunciation);
}
