use super::{Cell, Framebuffer};

#[derive(Clone, Debug, PartialEq)]
pub enum Changeset {
    Add { x: u16, y: u16, cell: Cell },
    Remove { x: u16, y: u16 },
    Update { x: u16, y: u16, cell: Cell },
}

impl std::fmt::Display for Changeset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Changeset::Add { x, y, cell } => write!(f, "[+] ({x}, {y}) - {cell:?}"),
            Changeset::Remove { x, y } => write!(f, "[-] ({x}, {y})"),
            Changeset::Update { x, y, cell } => write!(f, "[~] ({x}, {y}) - {cell:?}"),
        }
    }
}

pub type Changesets = Vec<Changeset>;

pub fn compare(a: &Framebuffer, b: &Framebuffer) -> Changesets {
    assert!(a.width() == b.width(), "width doesn't match");
    assert!(a.height() == b.height(), "height doesn't match");

    let mut changesets = Vec::new();

    a.iter()
        .zip(b.iter())
        .for_each(|(((x_a, y_a), cell_a), ((x_b, y_b), cell_b))| {
            assert!(x_a == x_b);
            assert!(y_a == y_b);

            use self::Changeset::*;

            match (cell_a, cell_b) {
                (Cell::Empty, Cell::Filled { .. }) => {
                    println!("add");
                    let change = Add {
                        x: x_a,
                        y: y_a,
                        cell: cell_b.clone(),
                    };

                    changesets.push(change);
                }
                (Cell::Filled { .. }, Cell::Empty) => {
                    println!("empty");
                    let change = Remove { x: x_a, y: y_a };
                    changesets.push(change);
                }
                (Cell::Filled { .. }, Cell::Filled { .. }) => {
                    if cell_a != cell_b {
                        println!("update");
                        let change = Update {
                            x: x_a,
                            y: y_a,
                            cell: cell_b.clone(),
                        };

                        changesets.push(change);
                    }
                }
                (Cell::Empty, Cell::Empty) => {}
            };
        });

    changesets
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::terminal::{bg, fg, Color::*};

    #[test]
    fn compare_same_framebuffer() {
        let mut fb = Framebuffer::new(2, 4);

        fb.set(
            0,
            0,
            Cell::Filled {
                character: 'H',
                foreground: fg(Red),
                background: bg(Yellow),
            },
        );
        fb.set(
            1,
            1,
            Cell::Filled {
                character: 'P',
                foreground: fg(Red),
                background: bg(Yellow),
            },
        );

        let diff = compare(&fb, &fb.clone());

        assert_eq!(diff.len(), 0, "raw diff {:?}", diff);
    }

    #[test]
    fn compare_new_to_added_items() {
        let fb_a = Framebuffer::new(2, 2);
        let mut fb_b = Framebuffer::new(2, 2);

        fb_b.set(
            0,
            0,
            Cell::Filled {
                character: 'P',
                foreground: fg(Red),
                background: bg(Yellow),
            },
        );
        fb_b.set(
            0,
            1,
            Cell::Filled {
                character: 'H',
                foreground: fg(Red),
                background: bg(Yellow),
            },
        );

        let diff = compare(&fb_a, &fb_b);

        assert_eq!(diff.len(), 2);
    }

    #[test]
    #[should_panic(expected = "height doesn't match")]
    fn cant_compare_different_size_framebuffer() {
        let fb_a = Framebuffer::new(2, 4);
        let fb_b = Framebuffer::new(2, 5);

        compare(&fb_a, &fb_b);
    }

    #[test]
    fn compare_changed_framebuffer() {
        let mut fb_a = Framebuffer::new(2, 6);
        fb_a.set(
            0,
            0,
            Cell::Filled {
                character: 'P',
                foreground: fg(Red),
                background: bg(Yellow),
            },
        );
        fb_a.set(
            1,
            1,
            Cell::Filled {
                character: 'o',
                foreground: fg(Red),
                background: bg(Yellow),
            },
        ); // remove

        let mut fb_b = Framebuffer::new(2, 6);
        fb_b.set(
            0,
            0,
            Cell::Filled {
                character: '!',
                foreground: fg(Red),
                background: bg(Yellow),
            },
        ); // up
        fb_b.set(
            1,
            3,
            Cell::Filled {
                character: '1',
                foreground: fg(Red),
                background: bg(Yellow),
            },
        ); // add

        let diff = compare(&fb_a, &fb_b);

        assert_eq!(
            diff,
            vec![
                Changeset::Update {
                    x: 0,
                    y: 0,
                    cell: Cell::Filled {
                        character: '!',
                        foreground: fg(Red),
                        background: bg(Yellow),
                    }
                },
                Changeset::Remove { x: 1, y: 1 },
                Changeset::Add {
                    x: 1,
                    y: 3,
                    cell: Cell::Filled {
                        character: '1',
                        foreground: fg(Red),
                        background: bg(Yellow),
                    }
                }
            ],
        );
    }
}
