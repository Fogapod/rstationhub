use std::io;

use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
    Frame,
};

use crate::app::AppAction;
use crate::input::UserInput;
use crate::states::{AppState, StatelessList};
use crate::views::{Drawable, InputProcessor};

pub struct CommitView {
    // TODO:
    //   - on 1st launch: fetch N latest commits, save latest hash
    //   - on 2nd launch: read latest hash and fetch newer commits
    loaded: bool,

    state: StatelessList<ListState>,
}

impl CommitView {
    pub fn new() -> Self {
        Self {
            loaded: false,
            state: StatelessList::new(ListState::default(), false),
        }
    }
}

#[async_trait::async_trait]
impl InputProcessor for CommitView {
    async fn on_input(&mut self, input: &UserInput, app: &AppState) -> Option<AppAction> {
        self.state.on_input(input, app.commits.read().await.count())
    }
}

#[async_trait::async_trait]
impl Drawable for CommitView {
    async fn draw(
        &mut self,
        f: &mut Frame<CrosstermBackend<io::Stdout>>,
        area: Rect,
        app: &AppState,
    ) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(50), Constraint::Min(0)])
            .split(area);

        let commits = &app.commits.read().await.items;

        let items: Vec<ListItem> = commits
            .iter()
            .map(|c| ListItem::new(c.title.clone()))
            .collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("messages"))
            .highlight_style(Style::default().bg(Color::DarkGray));

        f.render_stateful_widget(list, chunks[0], &mut self.state.state);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)])
            .split(chunks[1]);

        if let Some(i) = self.state.state.selected() {
            let selected = &commits[i];

            let text = Text::from(format!("author: {}", selected.author.name));

            let par = Paragraph::new(text)
                .alignment(Alignment::Left)
                .block(Block::default().borders(Borders::ALL).title("author"))
                .wrap(Wrap { trim: true });

            f.render_widget(par, chunks[0]);
            f.render_widget(
                Block::default().borders(Borders::ALL).title("info"),
                chunks[1],
            );
        }

        if !self.loaded {
            let commits = app.commits.clone();

            tokio::task::spawn(async move { commits.write().await.load().await });

            self.loaded = true;
        }
    }
}
