use std::io::Write;

use crate::{
    framebuffer::{
        Cell,
        diff::{Changeset, Changesets},
    },
    terminal::{Command, Style},
};

pub trait Renderer {
    fn submit(&mut self, changesets: Changesets);
}

pub struct Terminal<W>
where
    W: Write,
{
    out: W,
}

impl<T: Write> Terminal<T> {
    pub fn new(out: T) -> Self {
        Self { out }
    }

    fn render(&mut self, change: Changeset) {
        match change {
            Changeset::Add { x, y, cell } => self.add(x, y, cell),
            Changeset::Remove { x, y } => self.remove(x, y),
            Changeset::Update { x, y, cell } => self.update(x, y, cell),
        }
    }

    fn add(&mut self, x: u16, y: u16, cell: Cell) {
        use Command::*;

        let ops = match cell {
            Cell::Empty => vec![
                MoveTo(x, y),
                ApplyStyle(Style::Reset),
                Write(String::from("\0")),
            ],
            Cell::Filled {
                character,
                foreground,
                background,
            } => {
                vec![
                    MoveTo(x, y),
                    ApplyStyle(foreground),
                    ApplyStyle(background),
                    Write(character.to_string()),
                ]
            }
        };

        self.apply(&ops)
    }

    fn remove(&mut self, x: u16, y: u16) {}

    fn update(&mut self, x: u16, y: u16, _cell: Cell) {}

    fn apply(&mut self, ops: &[Command]) {
        for op in ops {
            write!(self.out, "{}", op).unwrap()
        }
    }
}

impl<T: Write> Renderer for Terminal<T> {
    fn submit(&mut self, changesets: Changesets) {
        for change in changesets {
            self.render(change)
        }
    }
}
