use std::io::Write;

use crate::{
    framebuffer::{
        diff::{Changeset, Changesets},
        Cell,
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
            Changeset::Add { x, y, cell } => self.update(x, y, cell),
            Changeset::Remove { x, y } => self.remove(x, y),
            Changeset::Update { x, y, cell } => self.update(x, y, cell),
        }
    }

    fn remove(&mut self, x: u16, y: u16)  {
	self.apply(&empty_at(x, y))
    }

    fn update(&mut self, x: u16, y: u16, cell: Cell) {
        use Command::*;

        match cell {
            Cell::Empty => self.apply(&empty_at(x, y)),
            Cell::Filled {
                character,
                foreground,
                background,
            } => {
		self.apply(&[MoveTo(x, y),
			     ApplyStyle(foreground),
			     ApplyStyle(background),
			     Write(character.to_string())])
            }
        };
    }

    fn apply(&mut self, ops: &[Command]) {
        for op in ops {
            write!(self.out, "{}", op).unwrap()
        }
    }
}

fn empty_at(x:  u16, y: u16) -> [Command; 3] {
    use Command::*;
    
    [MoveTo(x, y),
     ApplyStyle(Style::Reset),
     Write(String::from("\0"))]
}

impl<T: Write> Renderer for Terminal<T> {
    fn submit(&mut self, changesets: Changesets) {
        for change in changesets {
            self.render(change)
        }
    }
}
