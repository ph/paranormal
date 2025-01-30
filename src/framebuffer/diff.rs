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

type Changesets = Vec<Changeset>;

pub fn compare(a: &Framebuffer, b: &Framebuffer) -> Changesets {
    let mut changesets = Vec::new();

    a.iter()
        .zip(b.iter())
        .for_each(|(((x_a, y_a), cell_a), ((x_b, y_b), cell_b))| {
            assert!(x_a == x_b);
            assert!(y_a == y_b);

            use Changeset::*;

            match (cell_a, cell_b) {
                (Cell::Empty, Cell::Filled { .. }) => {
                    let change = Add {
                        x: x_a,
                        y: y_a,
                        cell: cell_b.clone(),
                    };

                    changesets.push(change);
                }
                (Cell::Filled { .. }, Cell::Empty) => {
                    let change = Remove { x: x_a, y: y_a };
                    changesets.push(change);
                }
                (Cell::Filled { .. }, Cell::Filled { .. }) => {
                    if cell_a != cell_b {
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
    Vec::new()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::framebuffer::render;

    #[test]
    fn compare_same_framebuffer() {
        let mut fb = Framebuffer::new(2, 4);

        fb.set(0, 0, Cell::Filled { character: 'H' });
        fb.set(1, 1, Cell::Filled { character: 'P' });

        let diff = compare(&fb, &fb.clone());

        // let mut buf = String::new();
        // render(&fb, &mut buf).expect("ok");

        // assert_eq!(buf, "");

        assert_eq!(diff.len(), 0, "raw diff {:?}", diff);
    }

    // fn compare_changed_framebuffer() {
    //     let mut fb_a = Framebuffer::new(2, 4);

    //     fb_a.set(2, 3, Cell::Filled { character: 'P' });
    //     fb_a.set(2, 2, Cell::Filled { character: 'H' });

    //     let mut fb_b = Framebuffer::new(2, 4);

    //     fb_b.set(2, 2, Cell::Filled { character: '!' });
    //     fb_b.set(0, 2, Cell::Filled { character: 'A' });

    //     assert_eq!(compare(fb_a, fb_b).len(), 3);
    // }
}
