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