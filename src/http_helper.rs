use reqwest::blocking::Client;
use std::error::Error;
use std::io::{Write, copy};

pub fn download_html(url: &str) -> Result<String, reqwest::Error> {
    // Make the synchronous GET request
    let response = reqwest::blocking::get(url)?;

    // Ensure the server returned a success code (e.g., 200 OK)
    let response = response.error_for_status()?;

    // Extract the body as a String
    let html_content = response.text()?;

    Ok(html_content)
}

pub fn download_file<W: Write>(url: &str, dest: &mut W) -> Result<(), Box<dyn Error>> {
    // Using a Client is better practice for configuring timeouts or headers
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;

    // Make the request
    let mut response = client.get(url).send()?.error_for_status()?;

    // Copy the stream from the network response to the file
    // This is memory efficient as it streams the data in chunks
    copy(&mut response, dest)?;

    Ok(())
}

#[cfg(test)]
mod download_html_tests;

#[cfg(test)]
mod download_file_tests;
