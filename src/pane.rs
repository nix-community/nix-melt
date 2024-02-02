use eyre::Result;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{List, ListItem, ListState},
    Frame,
};
use time::{format_description::FormatItem, OffsetDateTime, UtcOffset};

use crate::{
    error::InvalidInput,
    lock::{Input, Resolve},
};

pub(crate) struct Pane {
    pub cursor: Input,
    pub len: usize,
    pub selected: usize,
    pub widget: List<'static>,
    pub state: ListState,
}

impl Pane {
    pub(crate) fn new(
        lock: &Resolve,
        offset: UtcOffset,
        time_format: &[FormatItem<'_>],
        cursor: Input,
    ) -> Result<Self> {
        let node = lock
            .get(&cursor)
            .ok_or_else(|| InvalidInput(cursor.clone()))?;

        let mut items = node
            .inputs
            .iter()
            .map(|(name, input)| {
                ListItem::new({
                    match input {
                        Input::Direct(x) => {
                            if x == name {
                                name.clone()
                            } else {
                                format!("{name}: {x}")
                            }
                        }
                        Input::Follow(xs) => {
                            let mut s = name.clone();
                            s.push_str(" → ");

                            let mut xs = xs.iter();
                            if let Some(x) = xs.next() {
                                s.push_str(x);
                                for x in xs {
                                    s.push('/');
                                    s.push_str(x);
                                }
                            } else {
                                s.push_str("<self>");
                            }

                            s
                        }
                    }
                })
            })
            .collect::<Vec<_>>();

        let len = node.inputs.len();
        let mut state = ListState::default();
        if len == 0 {
            items.push(ListItem::new("(no inputs)"));
        } else {
            state.select(Some(0));
        }

        if let Some(locked) = &node.locked {
            items.extend([
                ListItem::new(""),
                ListItem::new(format!("type: {}", locked.type_)),
            ]);

            if let Some(time) = locked.last_modified {
                items.push(ListItem::new(format!(
                    "lastModified: {}",
                    OffsetDateTime::from_unix_timestamp(time as i64)?
                        .to_offset(offset)
                        .format(time_format)?,
                )));
            }

            items.extend(
                locked
                    .fields
                    .iter()
                    .map(|(k, v)| ListItem::new(format!("{k}: {v}"))),
            );
        }

        let widget = List::new(items).highlight_style(
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        );

        Ok(Self {
            cursor,
            len,
            selected: 0,
            widget,
            state,
        })
    }

    pub(crate) fn render(&self, frame: &mut Frame, rect: Rect, middle: bool) -> ListState {
        let mut state = self.state.clone();
        frame.render_stateful_widget(
            if middle {
                self.widget
                    .clone()
                    .highlight_style(
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    )
                    .highlight_symbol("🞄 ")
            } else {
                self.widget.clone()
            },
            rect,
            &mut state,
        );
        state
    }
}
