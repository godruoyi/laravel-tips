use serde::de::{DeserializeOwned};

pub fn http_get<T: DeserializeOwned>(url: &str) -> anyhow::Result<T> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_static("laravel-tips"),
    );
    headers.insert(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_static("application/vnd.github.v3+json"),
    );
    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()?;

    let res = client.get(url).send()?.json::<T>()?;

    Ok(res)
}