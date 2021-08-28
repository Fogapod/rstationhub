mod app;
mod commits;
mod installations;
mod locations;
mod servers;

pub use app::AppState;
pub use commits::CommitState;
pub use installations::InstallationsState;
pub use locations::LocationsState;
pub use servers::ServersState;

use tui::widgets::{ListState, TableState};

use crate::app::AppAction;
use crate::input::UserInput;

// tui states look same, but do not implement trait, so I made one
pub trait TuiState {
    fn selected(&self) -> Option<usize>;
    fn select(&mut self, index: Option<usize>);
}

impl TuiState for ListState {
    fn selected(&self) -> Option<usize> {
        ListState::selected(self)
    }

    fn select(&mut self, index: Option<usize>) {
        ListState::select(self, index)
    }
}

impl TuiState for TableState {
    fn selected(&self) -> Option<usize> {
        TableState::selected(self)
    }

    fn select(&mut self, index: Option<usize>) {
        TableState::select(self, index)
    }
}

// state compatible with both table and list
pub struct StatelessList<T: TuiState> {
    pub state: T,
    looped: bool,
}

impl<T: TuiState> StatelessList<T> {
    pub fn new(state: T, looped: bool) -> Self {
        Self { state, looped }
    }

    pub fn select_next(&mut self, item_count: usize) {
        if item_count == 0 {
            self.state.select(None);
        } else {
            match self.selected() {
                None => self.state.select(Some(0)),
                Some(i) => {
                    if i < item_count - 1 {
                        self.state.select(Some(i + 1))
                    } else if self.looped {
                        self.state.select(Some(0))
                    }
                }
            }
        }
    }

    pub fn select_previous(&mut self, item_count: usize) {
        if item_count == 0 {
            self.state.select(None);
        } else {
            match self.state.selected() {
                None => self.state.select(Some(0)),
                Some(i) => {
                    if i != 0 {
                        self.state.select(Some(i - 1))
                    } else if self.looped {
                        self.state.select(Some(item_count - 1))
                    }
                }
            }
        }
    }

    pub fn select_first(&mut self, item_count: usize) {
        if item_count == 0 {
            self.state.select(None);
        } else {
            self.state.select(Some(0));
        }
    }

    pub fn select_last(&mut self, item_count: usize) {
        if item_count == 0 {
            self.state.select(None);
        } else {
            self.state.select(Some(item_count - 1));
        }
    }

    pub fn select_index(&mut self, index: usize) {
        self.state.select(Some(index))
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    pub fn selected(&self) -> Option<usize> {
        self.state.selected()
    }

    pub fn on_input(&mut self, input: &UserInput, item_count: usize) -> Option<AppAction> {
        match input {
            UserInput::Up => {
                self.select_previous(item_count);
                Some(AppAction::Accepted)
            }
            UserInput::Down => {
                self.select_next(item_count);
                Some(AppAction::Accepted)
            }
            UserInput::Back => {
                self.unselect();
                Some(AppAction::Accepted)
            }
            UserInput::Top => {
                self.select_first(item_count);
                Some(AppAction::Accepted)
            }
            UserInput::Bottom => {
                self.select_last(item_count);
                Some(AppAction::Accepted)
            }
            _ => None,
        }
    }
}
