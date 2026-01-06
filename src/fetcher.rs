//! HTTP fetcher for textfiles.com

use anyhow::{Context, Result};
use reqwest::blocking::Client;
use std::{thread, time::Duration};

pub struct Fetcher {
    client: Client,
}

impl Fetcher {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("TextfilesBrowser/1.0")
            .build()
            .context("Failed to create HTTP client")?;
        Ok(Self { client })
    }

    pub fn fetch(&self, url: &str) -> Result<String> {
        let mut last_err = None;
        for _ in 0..3 {
            match self.try_fetch(url) {
                Ok(body) => return Ok(body),
                Err(e) => {
                    last_err = Some(e);
                    thread::sleep(Duration::from_secs(1));
                }
            }
        }
        Err(last_err.unwrap())
    }

    fn try_fetch(&self, url: &str) -> Result<String> {
        let resp = self.client.get(url).send().context("Request failed")?;
        if !resp.status().is_success() {
            anyhow::bail!("HTTP {}", resp.status());
        }
        resp.text().context("Failed to read response")
    }
}
