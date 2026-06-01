use std::io::Cursor;

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
    let mut output_data = Cursor::new(Vec::<u8>::new());

    let mut server = mockito::Server::new();
    create_mockito_fixture_end_points(&mut server);

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
        String::from_utf8(output_data.into_inner()).unwrap(),
        "apple\tˈæp.əl\t[sound:apple_us.mp3]\norange\tˈɔːr.ɪndʒ\t[sound:orange_us.mp3]\nbanana\tbəˈnæn.ə\t[sound:banana_us.mp3]\n"
    );
    assert!(media_download_dir.path().join("apple_us.mp3").exists());
    assert!(media_download_dir.path().join("orange_us.mp3").exists());
    assert!(media_download_dir.path().join("banana_us.mp3").exists());
}

#[test]
fn should_skip_input_words_existing_in_output() {
    // Arrange
    let input_data = "\
#separator:tab
#html:false
apple
orange
banana
";
    let mut output_data = Cursor::new(Vec::<u8>::from("apple\nbanana\n")); // apple and banana already exist in output

    let mut server = mockito::Server::new();
    create_mockito_fixture_end_points(&mut server);

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
        String::from_utf8(output_data.into_inner()).unwrap(),
        "apple\nbanana\norange\tˈɔːr.ɪndʒ\t[sound:orange_us.mp3]\n"
    );
    assert!(!media_download_dir.path().join("apple_us.mp3").exists());
    assert!(media_download_dir.path().join("orange_us.mp3").exists());
    assert!(!media_download_dir.path().join("banana_us.mp3").exists());
}
