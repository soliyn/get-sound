use super::*;

#[test]
fn should_return_ok_result() {
    let mut server = mockito::Server::new();
    let url = "/media/english/uk_pron/u/uks/uksta/ukstarg004.mp3";
    let expected = vec![0x00_u8, 0xFF, 0x1A, 0x2B];

    let _m = server
        .mock("GET", url)
        .with_status(200)
        .with_header("Content-Type", "audio/mpeg")
        .with_body(&expected)
        .create();

    let mut dest = Vec::<u8>::new();

    let result = download_file(&format!("{}{}", &server.url(), url), &mut dest);

    assert!(result.is_ok());
    assert_eq!(dest, expected);
}
