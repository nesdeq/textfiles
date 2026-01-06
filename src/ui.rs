//! Terminal UI - Green phosphor CRT aesthetic

use crate::browser::{Browser, Content, Page};
use crate::parser::DirEntry;
use anyhow::Result;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap},
    Frame,
};

// Phosphor green palette
const GREEN_BRIGHT: Color = Color::Rgb(57, 255, 20);
const GREEN_NORMAL: Color = Color::Rgb(0, 200, 0);
const GREEN_DIM: Color = Color::Rgb(0, 140, 0);
const BLACK: Color = Color::Rgb(0, 10, 0);

const HEADER: &str = r#"
 ▄▄▄█████▓▓█████ ▒██   ██▒▄▄▄█████▓  █████▒██▓ ██▓    ▓█████   ██████
 ▓  ██▒ ▓▒▓█   ▀ ▒▒ █ █ ▒░▓  ██▒ ▓▒▓██   ▒▓██▒▓██▒    ▓█   ▀ ▒██    ▒
 ▒ ▓██░ ▒░▒███   ░░  █   ░▒ ▓██░ ▒░▒████ ░▒██▒▒██░    ▒███   ░ ▓██▄
 ░ ▓██▓ ░ ▒▓█  ▄  ░ █ █ ▒ ░ ▓██▓ ░ ░▓█▒  ░░██░▒██░    ▒▓█  ▄   ▒   ██▒
   ▒██▒ ░ ░▒████▒▒██▒ ▒██▒  ▒██▒ ░ ░▒█░   ░██░░██████▒░▒████▒▒██████▒▒
   ▒ ░░   ░░ ▒░ ░▒▒ ░ ░▓ ░  ▒ ░░    ▒ ░   ░▓  ░ ▒░▓  ░░░ ▒░ ░▒ ▒▓▒ ▒ ░
     ░     ░ ░  ░░░   ░▒ ░    ░     ░      ▒ ░░ ░ ▒  ░ ░ ░  ░░ ░▒  ░ ░
   ░         ░    ░    ░    ░       ░ ░    ▒ ░  ░ ░      ░   ░  ░  ░
             ░  ░ ░    ░                   ░      ░  ░   ░  ░      ░
══════════════════════════════ .COM ═══════════════════════════════════"#;

#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    Browser,
    Viewer,
}

pub struct App {
    pub browser: Browser,
    pub mode: Mode,
    pub entries: Vec<DirEntry>,
    pub list_state: ListState,
    pub title: String,
    pub text_lines: Vec<String>,
    pub scroll: usize,
    pub error: Option<String>,
    pub tick: u64,
}

impl App {
    pub fn new() -> Self {
        Self {
            browser: Browser::new().expect("Failed to create browser"),
            mode: Mode::Browser,
            entries: Vec::new(),
            list_state: ListState::default(),
            title: String::new(),
            text_lines: Vec::new(),
            scroll: 0,
            error: None,
            tick: 0,
        }
    }

    pub fn load_home(&mut self) -> Result<()> {
        self.navigate_to("http://textfiles.com/directory.html")
    }

    pub fn navigate_to(&mut self, url: &str) -> Result<()> {
        match self.browser.navigate(url) {
            Ok(page) => self.apply_page(page),
            Err(e) => self.error = Some(e.to_string()),
        }
        Ok(())
    }

    fn apply_page(&mut self, page: Page) {
        self.title = page.title;
        match page.content {
            Content::Directory(entries) => {
                self.entries = entries;
                self.list_state.select(if self.entries.is_empty() { None } else { Some(0) });
                self.mode = Mode::Browser;
            }
            Content::TextFile(text) => {
                self.text_lines = text.lines().map(String::from).collect();
                self.scroll = 0;
                self.mode = Mode::Viewer;
            }
        }
    }

    pub fn select(&mut self) -> Result<()> {
        if let Some(i) = self.list_state.selected() {
            if let Some(entry) = self.entries.get(i) {
                let url = entry.url.clone();
                self.navigate_to(&url)?;
            }
        }
        Ok(())
    }

    pub fn go_back(&mut self) -> Result<()> {
        if self.mode == Mode::Viewer {
            // Return to directory
            match self.browser.go_back() {
                Ok(Some(page)) => self.apply_page(page),
                Ok(None) => {}
                Err(e) => self.error = Some(e.to_string()),
            }
        } else if let Ok(Some(page)) = self.browser.go_back() {
            self.apply_page(page);
        }
        Ok(())
    }

    pub fn refresh(&mut self) -> Result<()> {
        match self.browser.refresh() {
            Ok(page) => self.apply_page(page),
            Err(e) => self.error = Some(e.to_string()),
        }
        Ok(())
    }

    pub fn next(&mut self) {
        if self.entries.is_empty() { return; }
        let i = self.list_state.selected().map(|i| (i + 1).min(self.entries.len() - 1)).unwrap_or(0);
        self.list_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = self.list_state.selected().map(|i| i.saturating_sub(1)).unwrap_or(0);
        self.list_state.select(Some(i));
    }

    pub fn page_down(&mut self) {
        if self.entries.is_empty() { return; }
        let i = self.list_state.selected().map(|i| (i + 20).min(self.entries.len() - 1)).unwrap_or(0);
        self.list_state.select(Some(i));
    }

    pub fn page_up(&mut self) {
        let i = self.list_state.selected().map(|i| i.saturating_sub(20)).unwrap_or(0);
        self.list_state.select(Some(i));
    }

    pub fn home(&mut self) {
        if !self.entries.is_empty() {
            self.list_state.select(Some(0));
        }
    }

    pub fn end(&mut self) {
        if !self.entries.is_empty() {
            self.list_state.select(Some(self.entries.len() - 1));
        }
    }

    pub fn scroll_up(&mut self, n: usize) {
        self.scroll = self.scroll.saturating_sub(n);
    }

    pub fn scroll_down(&mut self, n: usize) {
        self.scroll = (self.scroll + n).min(self.text_lines.len().saturating_sub(1));
    }

    pub fn scroll_home(&mut self) {
        self.scroll = 0;
    }

    pub fn scroll_end(&mut self) {
        self.scroll = self.text_lines.len().saturating_sub(1);
    }

    pub fn tick(&mut self) {
        self.tick = self.tick.wrapping_add(1);
    }
}

pub fn draw(frame: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(12),
            Constraint::Min(5),
            Constraint::Length(2),
        ])
        .split(frame.area());

    draw_header(frame, chunks[0], app);

    match app.mode {
        Mode::Browser => draw_browser(frame, chunks[1], app),
        Mode::Viewer => draw_viewer(frame, chunks[1], app),
    }

    draw_status(frame, chunks[2], app);

    if let Some(ref err) = app.error {
        draw_error(frame, frame.area(), err);
    }
}

fn draw_header(frame: &mut Frame, area: Rect, app: &App) {
    let color = if app.tick % 30 < 2 { GREEN_NORMAL } else { GREEN_BRIGHT };
    let lines: Vec<Line> = HEADER.lines()
        .map(|l| Line::from(Span::styled(l, Style::default().fg(color))))
        .collect();
    let p = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .style(Style::default().bg(BLACK));
    frame.render_widget(p, area);
}

fn draw_browser(frame: &mut Frame, area: Rect, app: &mut App) {
    let items: Vec<ListItem> = app.entries.iter().enumerate().map(|(i, e)| {
        let selected = app.list_state.selected() == Some(i);
        let icon = if e.is_dir { "<DIR>" } else { "     " };

        let (name_style, desc_style) = if selected {
            (Style::default().fg(BLACK).bg(GREEN_BRIGHT).add_modifier(Modifier::BOLD),
             Style::default().fg(BLACK).bg(GREEN_BRIGHT))
        } else if e.is_dir {
            (Style::default().fg(GREEN_BRIGHT),
             Style::default().fg(GREEN_DIM))
        } else {
            (Style::default().fg(GREEN_NORMAL),
             Style::default().fg(GREEN_DIM))
        };

        let line = if e.description.is_empty() {
            Line::from(vec![
                Span::styled(format!("{} {}", icon, e.name), name_style),
            ])
        } else {
            Line::from(vec![
                Span::styled(format!("{} {}", icon, e.name), name_style),
                Span::styled(format!(" - {}", e.description), desc_style),
            ])
        };

        ListItem::new(line)
    }).collect();

    let list = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_set(border::PLAIN)
            .border_style(Style::default().fg(GREEN_DIM))
            .title(Span::styled(
                format!(" {} ", app.title),
                Style::default().fg(GREEN_BRIGHT).add_modifier(Modifier::BOLD)
            ))
            .style(Style::default().bg(BLACK)))
        .highlight_symbol("> ");

    let mut scrollbar_state = ScrollbarState::new(app.entries.len())
        .position(app.list_state.selected().unwrap_or(0));

    frame.render_stateful_widget(list, area, &mut app.list_state);
    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .style(Style::default().fg(GREEN_DIM)),
        area,
        &mut scrollbar_state
    );
}

fn draw_viewer(frame: &mut Frame, area: Rect, app: &App) {
    let height = area.height.saturating_sub(2) as usize;
    let lines: Vec<Line> = app.text_lines.iter()
        .skip(app.scroll)
        .take(height)
        .map(|l| Line::from(Span::styled(l.as_str(), Style::default().fg(GREEN_NORMAL))))
        .collect();

    let total = app.text_lines.len();
    let pct = if total > 0 { ((app.scroll + height).min(total) * 100) / total } else { 100 };

    let p = Paragraph::new(lines)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_set(border::PLAIN)
            .border_style(Style::default().fg(GREEN_DIM))
            .title(Span::styled(
                format!(" {} [{}%] ", app.title, pct),
                Style::default().fg(GREEN_BRIGHT).add_modifier(Modifier::BOLD)
            ))
            .style(Style::default().bg(BLACK)))
        .wrap(Wrap { trim: false });

    let mut scrollbar_state = ScrollbarState::new(total).position(app.scroll);

    frame.render_widget(p, area);
    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .style(Style::default().fg(GREEN_DIM)),
        area,
        &mut scrollbar_state
    );
}

fn draw_status(frame: &mut Frame, area: Rect, app: &App) {
    let mode = if app.mode == Mode::Browser { "BROWSE" } else { "VIEW" };
    let back = if app.browser.can_go_back() { "←BACK " } else { "" };

    let left = format!(" {} │ {}", mode, app.browser.current_url);
    let right = format!("{}q:quit ", back);
    let width = area.width as usize;
    let pad = width.saturating_sub(left.len()).saturating_sub(right.len());

    let line = Line::from(vec![
        Span::styled(&left, Style::default().fg(GREEN_BRIGHT)),
        Span::raw(" ".repeat(pad)),
        Span::styled(&right, Style::default().fg(GREEN_DIM)),
    ]);

    frame.render_widget(Paragraph::new(line).style(Style::default().bg(BLACK)), area);
}

fn draw_error(frame: &mut Frame, area: Rect, msg: &str) {
    let w = 60.min(area.width.saturating_sub(4));
    let h = 7;
    let x = (area.width - w) / 2;
    let y = (area.height - h) / 2;
    let rect = Rect::new(x, y, w, h);

    frame.render_widget(Clear, rect);

    let lines = vec![
        Line::from(""),
        Line::from(Span::styled("ERROR", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))),
        Line::from(""),
        Line::from(Span::styled(msg, Style::default().fg(GREEN_BRIGHT))),
        Line::from(""),
        Line::from(Span::styled("Press any key...", Style::default().fg(GREEN_DIM))),
    ];

    let p = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_set(border::PLAIN)
            .border_style(Style::default().fg(Color::Red))
            .style(Style::default().bg(BLACK)));

    frame.render_widget(p, rect);
}
