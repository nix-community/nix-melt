mod cli;
mod error;
mod lock;
mod pane;
mod state;

use std::fs::File;

use clap::Parser;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use eyre::Result;
use time::format_description;

use crate::{cli::Opts, state::State};

fn main() -> Result<()> {
    color_eyre::install()?;
    let opts = Opts::parse();

    let lock = if opts.path.is_dir() {
        opts.path.join("flake.lock")
    } else {
        opts.path
    };

    let mut state = State::new(
        serde_json::from_reader(File::open(lock)?)?,
        format_description::parse_borrowed::<2>(&opts.time_format)?,
    )?;
    state.render()?;

    while let Ok(ev) = event::read() {
        let Event::Key(KeyEvent { code, modifiers, .. }) = ev else {
            continue;
        };

        match code {
            KeyCode::Char('q') => break,
            KeyCode::Char('c') if modifiers == KeyModifiers::CONTROL => break,
            KeyCode::Char('h') | KeyCode::Left => {
                if state.current != 0 {
                    state.current -= 1;
                    state.render()?;
                }
            }
            KeyCode::Char('j') | KeyCode::Down => {
                state.select(|i| i + 1)?;
            }
            KeyCode::Char('k') | KeyCode::Up => {
                state.select(|i| i.saturating_sub(1))?;
            }
            KeyCode::Char('l') | KeyCode::Right => {
                if state.current + 1 < state.panes.len() {
                    state.current += 1;
                    state.render()?;
                }
            }
            _ => {}
        }
    }

    Ok(())
}
