use crate::{cambridge_dictionary::WordPronunciation, http_helper};

use super::*;
use rstest::rstest;

#[rstest]
#[case(PronunciationRegion::Uk)]
#[case(PronunciationRegion::Us)]
fn should_return_ok_and_download_corresponding_sound(
    #[case] preferred_pronunciation: PronunciationRegion,
) {
    let media_download_dir = tempfile::tempdir().unwrap();
    let mut output_data = Vec::<u8>::new();
    let mut server = mockito::Server::new();
    let word_info = WordInfo {
        word: "apple".to_string(),
        pronunciations: vec![
            get_uk_pronunciation(&server.url()),
            get_us_pronunciation(&server.url()),
        ],
    };
    let expected = vec![0x00_u8, 0xFF, 0x1A, 0x2B];
    create_mockito_mpeg_end_point(
        &mut server,
        "/media/english/uk_pron/u/uka/ukapp/ukappen014.mp3",
        &expected,
    );
    create_mockito_mpeg_end_point(
        &mut server,
        "/media/english/us_pron/a/app/apple/apple.mp3",
        &expected,
    );

    let result = process_word(
        &word_info,
        &preferred_pronunciation,
        &media_download_dir,
        &mut output_data,
        &http_helper::HttpHelper::new(),
    );

    assert!(result.is_ok());
    assert!(
        media_download_dir
            .path()
            .join(format!("apple_{}.mp3", preferred_pronunciation))
            .exists()
    );
    assert_eq!(
        String::from_utf8(output_data).unwrap(),
        format!(
            "apple\tˈæp.əl\t[sound:apple_{}.mp3]\n",
            preferred_pronunciation
        )
    );
}

#[rstest]
fn should_return_ok_and_download_uk_sound() {
    let media_download_dir = tempfile::tempdir().unwrap();
    let mut output_data = Vec::<u8>::new();
    let mut server = mockito::Server::new();
    let word_info = WordInfo {
        word: "apple".to_string(),
        pronunciations: vec![get_uk_pronunciation(&server.url())],
    };
    let expected = vec![0x00_u8, 0xFF, 0x1A, 0x2B];
    create_mockito_mpeg_end_point(
        &mut server,
        "/media/english/uk_pron/u/uka/ukapp/ukappen014.mp3",
        &expected,
    );

    let result = process_word(
        &word_info,
        &PronunciationRegion::Us,
        &media_download_dir,
        &mut output_data,
        &http_helper::HttpHelper::new(),
    );

    assert!(result.is_ok());
    assert!(media_download_dir.path().join("apple_uk.mp3").exists());
    assert_eq!(
        String::from_utf8(output_data).unwrap(),
        "apple\tˈæp.əl\t[sound:apple_uk.mp3]\n"
    );
}

#[rstest]
fn should_return_ok_and_download_us_sound() {
    let media_download_dir = tempfile::tempdir().unwrap();
    let mut output_data = Vec::<u8>::new();
    let mut server = mockito::Server::new();
    let word_info = WordInfo {
        word: "apple".to_string(),
        pronunciations: vec![get_us_pronunciation(&server.url())],
    };
    let expected = vec![0x00_u8, 0xFF, 0x1A, 0x2B];
    create_mockito_mpeg_end_point(
        &mut server,
        "/media/english/us_pron/a/app/apple/apple.mp3",
        &expected,
    );

    let result = process_word(
        &word_info,
        &PronunciationRegion::Uk,
        &media_download_dir,
        &mut output_data,
        &http_helper::HttpHelper::new(),
    );

    assert!(result.is_ok());
    assert!(media_download_dir.path().join("apple_us.mp3").exists());
    assert_eq!(
        String::from_utf8(output_data).unwrap(),
        "apple\tˈæp.əl\t[sound:apple_us.mp3]\n"
    );
}

#[rstest]
fn should_return_error_when_no_pronunciation_found() {
    let media_download_dir = tempfile::tempdir().unwrap();
    let mut output_data = Vec::<u8>::new();
    let word_info = WordInfo {
        word: "apple".to_string(),
        pronunciations: vec![],
    };

    let result = process_word(
        &word_info,
        &PronunciationRegion::Uk,
        &media_download_dir,
        &mut output_data,
        &http_helper::HttpHelper::new(),
    );

    assert!(result.is_err());
    assert_eq!(String::from_utf8(output_data).unwrap(), "apple\t\t\n");
}

fn get_uk_pronunciation(base_url: &str) -> WordPronunciation {
    WordPronunciation {
        region: PronunciationRegion::Uk,
        ipa_transcription: Some("ˈæp.əl".to_string()),
        audio_mpeg: Some(format!(
            "{}/media/english/uk_pron/u/uka/ukapp/ukappen014.mp3",
            base_url
        )),
    }
}

fn get_us_pronunciation(base_url: &str) -> WordPronunciation {
    WordPronunciation {
        region: PronunciationRegion::Us,
        ipa_transcription: Some("ˈæp.əl".to_string()),
        audio_mpeg: Some(format!(
            "{}/media/english/us_pron/a/app/apple/apple.mp3",
            base_url
        )),
    }
}
