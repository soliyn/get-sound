use super::*;

#[test]
fn should_return_ok_result_when_status_200() {
    let mut server = mockito::Server::new();
    let url = "/english/table";

    let _m = server
        .mock("GET", url)
        .with_status(200)
        .with_header("Content-Type", "text/html; charset=utf-8")
        .with_body("table")
        .create();

    let helper = HttpHelper::new();
    let result = helper.download_html(&format!("{}{}", &server.url(), url));

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "table");
}

#[test]
fn should_return_err_result_when_status_500() {
    let mut server = mockito::Server::new();
    let url = "/english/table";

    // Mock a 500 Internal Server Error
    let _m = server.mock("GET", url).with_status(500).create();

    let helper = HttpHelper::new();
    let result = helper.download_html(&format!("{}{}", &server.url(), url));

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err, DownloadError::NotOkStatus(reqwest::StatusCode::INTERNAL_SERVER_ERROR));
}

#[test]
fn should_return_err_result_when_status_404_not_found() {
    let mut server = mockito::Server::new();
    let url = "/english/table";

    let _m = server.mock("GET", url).with_status(404).create();

    let helper = HttpHelper::new();
    let result = helper.download_html(&format!("{}{}", &server.url(), url));

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err, DownloadError::NotOkStatus(reqwest::StatusCode::NOT_FOUND));
}

#[test]
fn should_return_err_result_when_status_302_redirect() {
    let mut server = mockito::Server::new();
    let url = "/english/table";

    let _m = server.mock("GET", url).with_status(302).create();

    let helper = HttpHelper::new();
    let result = helper.download_html(&format!("{}{}", &server.url(), url));

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err, DownloadError::NotOkStatus(reqwest::StatusCode::FOUND));
}
