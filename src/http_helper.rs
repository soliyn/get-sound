use log::{debug, warn};
use reqwest::StatusCode;
use reqwest::blocking::Client;
use reqwest::redirect::Policy;
use std::io::{Write, copy};
use anyhow::Result;

pub struct HttpHelper {
    client: Client,
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum DownloadError {
    #[error("not OK status code: {0}")]
    NotOkStatus(StatusCode),

    #[error("request failed: {0}")]
    RequestFail(String),
}

impl From<reqwest::Error> for DownloadError {
    fn from(value: reqwest::Error) -> Self {
        DownloadError::RequestFail(format!("{value}"))
    }
}

#[cfg_attr(test, mockall::automock)]
pub trait Downloader {
    fn download_html(&self, url: &str) -> Result<String, DownloadError>;

    fn download_file<W: Write + 'static>(&self, url: &str, dest: &mut W) -> Result<()>;
}

impl HttpHelper {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .redirect(Policy::none())
            .build()
            .unwrap();
        Self { client }
    }
}

impl Downloader for HttpHelper {
    fn download_html(&self, url: &str) -> Result<String, DownloadError> {
        let response = self.client.get(url).send()?;
        debug!(
            "Download html. Url: {}. Response status: {}",
            url,
            response.status()
        );

        if !response.status().is_success() {
            warn!("Not OK status when downloading html: {}", response.status());
            return Err(DownloadError::NotOkStatus(response.status()));
        }

        let html_content = response.text()?;

        Ok(html_content)
    }

    fn download_file<W: Write>(&self, url: &str, dest: &mut W) -> Result<()> {
        // Make the request
        let mut response = self.client.get(url).send()?.error_for_status()?;

        // Copy the stream from the network response to the file
        // This is memory efficient as it streams the data in chunks
        copy(&mut response, dest)?;

        Ok(())
    }
}

#[cfg(test)]
mod download_html_tests;

#[cfg(test)]
mod download_file_tests;
