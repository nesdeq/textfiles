//! Browser state and navigation

use crate::fetcher::Fetcher;
use crate::parser::{self, DirEntry};
use anyhow::Result;

const HOME_URL: &str = "http://textfiles.com/directory.html";

#[derive(Debug, Clone)]
pub enum Content {
    Directory(Vec<DirEntry>),
    TextFile(String),
}

#[derive(Debug, Clone)]
pub struct Page {
    pub title: String,
    pub content: Content,
}

pub struct Browser {
    fetcher: Fetcher,
    pub history: Vec<String>,
    pub current_url: String,
}

impl Browser {
    pub fn new() -> Result<Self> {
        Ok(Self {
            fetcher: Fetcher::new()?,
            history: Vec::new(),
            current_url: HOME_URL.to_string(),
        })
    }

    pub fn load(&mut self, url: &str) -> Result<Page> {
        let body = self.fetcher.fetch(url)?;
        let start: String = body.trim_start().chars().take(15).collect();
        let lower = start.to_lowercase();
        let is_html = lower.starts_with("<!doctype") || lower.starts_with("<html");

        if is_html {
            let entries = if url.ends_with("directory.html") {
                parser::parse_directory_html(&body)
            } else {
                parser::parse_file_listing(&body, url)
            };

            let title = parser::parse_page_title(&body).unwrap_or_else(|| {
                url.trim_end_matches('/').split('/').last()
                    .unwrap_or("TEXTFILES.COM").to_uppercase()
            });

            Ok(Page { title, content: Content::Directory(entries) })
        } else {
            let title = url.split('/').last().unwrap_or("file").to_string();
            Ok(Page { title, content: Content::TextFile(body) })
        }
    }

    pub fn navigate(&mut self, url: &str) -> Result<Page> {
        self.history.push(self.current_url.clone());
        self.current_url = url.to_string();
        self.load(url)
    }

    pub fn go_back(&mut self) -> Result<Option<Page>> {
        if let Some(prev) = self.history.pop() {
            self.current_url = prev.clone();
            Ok(Some(self.load(&prev)?))
        } else {
            Ok(None)
        }
    }

    pub fn refresh(&mut self) -> Result<Page> {
        self.load(&self.current_url.clone())
    }

    pub fn can_go_back(&self) -> bool {
        !self.history.is_empty()
    }
}
