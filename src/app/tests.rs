#[cfg(test)]
mod get_audio_file_name_tests;
#[cfg(test)]
mod run_internal_tests;

#[cfg(test)]
mod process_word_tests;

use mockito::ServerGuard;

pub use super::*;


// test helpers

fn create_mockito_html_end_point(server: &mut ServerGuard, word: &str) {
    server
        .mock("GET", format!("/dictionary/english/{}", word).as_str())
        .with_status(200)
        .with_header("Content-Type", "text/html; charset=utf-8")
        .with_body_from_file(format!("tests/fixtures/{}_cambridge.html", word))
        .create();
}
fn create_mockito_mpeg_end_point(server: &mut ServerGuard, url: &str, expected: &[u8]) {
    server
        .mock("GET", url)
        .with_status(200)
        .with_header("Content-Type", "audio/mpeg")
        .with_body(expected)
        .create();
}

fn create_mockito_fixture_end_points(server: &mut ServerGuard) {
    let expected = vec![0x00_u8, 0xFF, 0x1A, 0x2B];
    create_mockito_html_end_point(server, "apple");
    create_mockito_mpeg_end_point(
        server,
        "/media/english/uk_pron/u/uka/ukapp/ukappen014.mp3",
        &expected,
    );
    create_mockito_mpeg_end_point(
        server,
        "/media/english/us_pron/a/app/apple/apple.mp3",
        &expected,
    );

    create_mockito_html_end_point(server, "orange");
    create_mockito_mpeg_end_point(
        server,
        "/media/english/uk_pron/u/uko/ukora/ukorang001.mp3",
        &expected,
    );
    create_mockito_mpeg_end_point(
        server,
        "/media/english/us_pron/e/eus/eus75/eus75268.mp3",
        &expected,
    );
    create_mockito_html_end_point(server, "banana");
    create_mockito_mpeg_end_point(
        server,
        "/media/english/uk_pron/u/ukb/ukbal/ukballs018.mp3",
        &expected,
    );
    create_mockito_mpeg_end_point(
        server,
        "/media/english/us_pron/b/ban/banan/banana.mp3",
        &expected,
    );

}