use std::io::{stdout, Stdout};

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use eyre::Result;
use ratatui::{backend::CrosstermBackend, layout::Rect, Terminal};
use time::{format_description::FormatItem, UtcOffset};

use crate::{
    error::{IndexOutOfBounds, InvalidInput},
    lock::{Input, Lock, Resolve},
    pane::Pane,
};

pub(crate) struct State<'a> {
    pub current: usize,
    pub panes: Vec<Pane>,
    lock: Resolve,
    tz: UtcOffset,
    time_format: Vec<FormatItem<'a>>,
    term: Terminal<CrosstermBackend<Stdout>>,
}

impl<'a> State<'a> {
    pub(crate) fn new(lock: Lock, time_format: Vec<FormatItem<'a>>) -> Result<Self> {
        enable_raw_mode()?;
        let mut stdout = stdout();
        stdout.execute(EnterAlternateScreen)?;

        let lock = lock.resolve()?;
        let tz = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
        let pane = Pane::new(&lock, tz, &time_format, Input::Follow(Vec::new()))?;

        Ok(Self {
            current: 0,
            panes: vec![pane],
            lock,
            tz,
            time_format,
            term: Terminal::new(CrosstermBackend::new(stdout))?,
        })
    }

    pub(crate) fn select(&mut self, f: impl Fn(usize) -> usize) -> Result<()> {
        let pane = &mut self
            .panes
            .get_mut(self.current)
            .ok_or(IndexOutOfBounds(self.current))?;

        let new = f(pane.selected).clamp(0, pane.len.saturating_sub(1));

        if new != pane.selected {
            pane.selected = new;
            pane.state.select(Some(new));
            self.render()?;
        }

        Ok(())
    }

    pub(crate) fn render(&mut self) -> Result<()> {
        let m = self
            .panes
            .get(self.current)
            .ok_or(IndexOutOfBounds(self.current))?;

        if let Some((_, cursor)) = self
            .lock
            .get(&m.cursor)
            .ok_or_else(|| InvalidInput(m.cursor.clone()))?
            .inputs
            .get_index(m.selected)
        {
            match self.panes.get(self.current + 1) {
                Some(pane) => {
                    if cursor != &pane.cursor {
                        self.panes[self.current + 1] = self.new_pane(cursor.clone())?;
                    }
                }
                _ => {
                    self.panes.push(self.new_pane(cursor.clone())?);
                }
            }
        } else {
            self.panes.truncate(self.current + 1);
        }

        let r = self.panes.get(self.current + 1);
        let m = &self.panes[self.current];
        let l = self
            .current
            .checked_sub(1)
            .and_then(|pane| self.panes.get(pane));

        let mut l_state = None;
        let mut m_state = None;
        let mut r_state = None;

        self.term.draw(|frame| {
            let rect = frame.size();
            let l_x = rect.x;
            let l_w = rect.width / 4;
            let m_x = l_x + l_w;
            let m_w = rect.width / 3;
            let r_x = m_x + m_w;
            let r_w = rect.right() - r_x;

            if let Some(l) = l {
                l_state =
                    Some(l.render(frame, Rect::new(l_x, rect.y, l_w - 1, rect.height), false));
            }
            m_state = Some(m.render(frame, Rect::new(m_x, rect.y, m_w - 1, rect.height), true));
            if let Some(r) = r {
                r_state = Some(r.render(frame, Rect::new(r_x, rect.y, r_w, rect.height), false));
            }
        })?;

        if let Some(state) = l_state {
            self.panes[self.current - 1].state = state;
        }
        if let Some(state) = m_state {
            self.panes[self.current].state = state;
        }
        if let Some(state) = r_state {
            self.panes[self.current + 1].state = state;
        }

        Ok(())
    }

    fn new_pane(&self, cursor: Input) -> Result<Pane> {
        Pane::new(&self.lock, self.tz, &self.time_format, cursor)
    }
}

impl Drop for State<'_> {
    fn drop(&mut self) {
        let _ = self.term.backend_mut().execute(LeaveAlternateScreen);
        let _ = disable_raw_mode();
    }
}
