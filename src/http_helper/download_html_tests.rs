use super::*;

#[test]
fn should_return_ok_result() {
    let mut server = mockito::Server::new();
    let url = "/english/table";

    let _m = server
        .mock("GET", url)
        .with_status(200)
        .with_header("Content-Type", "text/html; charset=utf-8")
        .with_body("table")
        .create();

    let result = download_html(&format!("{}{}", &server.url(), url));

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "table");
}

#[test]
fn should_return_err_result_status_500() {
    let mut server = mockito::Server::new();
    let url = "/english/table";

    // Mock a 500 Internal Server Error
    let _m = server.mock("GET", url).with_status(500).create();

    let result = download_html(&format!("{}{}", &server.url(), url));

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.status().unwrap() == 500);
}

#[test]
fn should_return_err_result_status_404_not_found() {
    let mut server = mockito::Server::new();
    let url = "/english/table";

    let _m = server.mock("GET", url).with_status(404).create();

    let result = download_html(&format!("{}{}", &server.url(), url));

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.status().unwrap() == 404);
}
