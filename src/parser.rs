//! HTML parser for textfiles.com

use scraper::{Html, Selector, ElementRef};

#[derive(Debug, Clone)]
pub struct DirEntry {
    pub name: String,
    pub url: String,
    pub description: String,
    pub is_dir: bool,
}

pub fn parse_directory_html(html: &str) -> Vec<DirEntry> {
    // directory.html has: <B><A HREF="dirname">Display Name</A></B><BR><I>Description</I>
    let document = Html::parse_document(html);
    let td_selector = Selector::parse("td").unwrap();
    let a_selector = Selector::parse("a").unwrap();
    let i_selector = Selector::parse("i").unwrap();

    let mut entries = Vec::new();

    for td in document.select(&td_selector) {
        // Find link in this TD
        let link = match td.select(&a_selector).next() {
            Some(l) => l,
            None => continue,
        };

        let href = match link.value().attr("href") {
            Some(h) => h,
            None => continue,
        };

        // Only directory links (no dots, slashes, etc)
        if href.contains('.') || href.contains('/') || href.contains(':') || href.is_empty() {
            continue;
        }

        let name = link.text().collect::<String>().trim().to_string();
        if name.is_empty() || name.len() > 50 {
            continue;
        }

        // Get description from <I> tag in same TD
        let description = td.select(&i_selector)
            .next()
            .map(|i| i.text().collect::<String>().trim().to_string())
            .unwrap_or_default();

        let url = format!("http://textfiles.com/{}/", href);

        entries.push(DirEntry {
            name,
            url,
            description,
            is_dir: true,
        });
    }

    entries
}

pub fn parse_file_listing(html: &str, base_url: &str) -> Vec<DirEntry> {
    // File listings: <TR><TD><A HREF="file.txt">file.txt</A></TD><TD>size</TD><TD>description</TD></TR>
    let document = Html::parse_document(html);
    let tr_selector = Selector::parse("tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();
    let a_selector = Selector::parse("a").unwrap();

    let mut entries = Vec::new();
    let base = base_url.trim_end_matches('/');

    for tr in document.select(&tr_selector) {
        let tds: Vec<ElementRef> = tr.select(&td_selector).collect();
        if tds.is_empty() {
            continue;
        }

        // First TD should have the link
        let link = match tds[0].select(&a_selector).next() {
            Some(l) => l,
            None => continue,
        };

        let href = match link.value().attr("href") {
            Some(h) => h,
            None => continue,
        };

        // Skip parent, queries, absolute URLs
        if href == "../" || href.starts_with('?') || href.starts_with("http") || href.starts_with('/') {
            continue;
        }

        let name = link.text().collect::<String>().trim().to_string();
        if name.is_empty() || name == "Name" || name == "Filename" {
            continue;
        }

        // Description is in the last TD (usually 3rd)
        let description = if tds.len() >= 3 {
            tds[tds.len() - 1].text().collect::<String>().trim().to_string()
        } else {
            String::new()
        };

        let url = format!("{}/{}", base, href);
        let is_dir = href.ends_with('/');

        entries.push(DirEntry {
            name: name.trim_end_matches('/').to_string(),
            url,
            description,
            is_dir,
        });
    }

    entries
}

pub fn parse_page_title(html: &str) -> Option<String> {
    let document = Html::parse_document(html);
    let title_selector = Selector::parse("title").ok()?;
    document
        .select(&title_selector)
        .next()
        .map(|t| t.text().collect::<String>().trim().to_string())
}
