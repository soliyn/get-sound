use super::*;

#[test]
fn should_process_all_input_words() {
    // Arrange
    let input_data = "\
#separator:tab
#html:false
apple
orange
banana
";
    let mut output_data = Vec::<u8>::new();

    let mut server = mockito::Server::new();
    let expected = vec![0x00_u8, 0xFF, 0x1A, 0x2B];
    create_mockito_html_end_point(&mut server, "apple");
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

    create_mockito_html_end_point(&mut server, "orange");
    create_mockito_mpeg_end_point(
        &mut server,
        "/media/english/uk_pron/u/uko/ukora/ukorang001.mp3",
        &expected,
    );
    create_mockito_mpeg_end_point(
        &mut server,
        "/media/english/us_pron/e/eus/eus75/eus75268.mp3",
        &expected,
    );
    create_mockito_html_end_point(&mut server, "banana");
    create_mockito_mpeg_end_point(
        &mut server,
        "/media/english/uk_pron/u/ukb/ukbal/ukballs018.mp3",
        &expected,
    );
    create_mockito_mpeg_end_point(
        &mut server,
        "/media/english/us_pron/b/ban/banan/banana.mp3",
        &expected,
    );

    let server_url = server.url();

    let dict = CambridgeDictionary::new(server_url);

    let media_download_dir = tempfile::tempdir().unwrap();

    // Act
    App::run_internal(RunInternalParams {
        input_file: input_data.as_bytes(),
        word_column_index: 0,
        output_file: &mut output_data,
        dict,
        language: Language::English,
        http_helper: HttpHelper::new(),
        media_download_dir: media_download_dir.path(),
        preferred_pronunciation: PronunciationRegion::Us,
    });

    // Assert
    assert_eq!(
        String::from_utf8(output_data).unwrap(),
        "apple\tˈæp.əl\t[sound:apple_us.mp3]\norange\tˈɔːr.ɪndʒ\t[sound:orange_us.mp3]\nbanana\tbəˈnæn.ə\t[sound:banana_us.mp3]\n"
    );
    assert!(media_download_dir.path().join("apple_us.mp3").exists());
    assert!(media_download_dir.path().join("orange_us.mp3").exists());
    assert!(media_download_dir.path().join("banana_us.mp3").exists());
}
