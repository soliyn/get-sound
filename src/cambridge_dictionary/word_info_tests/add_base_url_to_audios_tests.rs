use super::*;

#[test]
fn should_add_base_url_prefix() {
    const DEFAULT_BASE_URL: &str = "https://dictionary.cambridge.org";
    let cd = CambridgeDictionary::new(DEFAULT_BASE_URL.to_string());
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
