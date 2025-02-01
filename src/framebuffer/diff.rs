use tracing::trace;

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
    assert!(a.width() == b.width(), "width doesn't match");
    assert!(a.height() == b.height(), "height doesn't match");

    let mut changesets = Vec::new();

    a.iter()
        .zip(b.iter())
        .for_each(|(((x_a, y_a), cell_a), ((x_b, y_b), cell_b))| {
            assert!(x_a == x_b);
            assert!(y_a == y_b);

            println!(
                "a: {:?}, cell_a: {:?} --  b: {:?}, cell_b: {:?}",
                (x_a, y_a),
                (x_b, y_b),
                cell_a,
                cell_b
            );

            use Changeset::*;

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
    use crate::framebuffer::render;

    #[test]
    fn compare_same_framebuffer() {
        let mut fb = Framebuffer::new(2, 4);

        fb.set(0, 0, Cell::Filled { character: 'H' });
        fb.set(1, 1, Cell::Filled { character: 'P' });

        let diff = compare(&fb, &fb.clone());

        assert_eq!(diff.len(), 0, "raw diff {:?}", diff);
    }

    #[test]
    fn compare_new_to_added_items() {
        let mut fb_a = Framebuffer::new(2, 2);
        let mut fb_b = Framebuffer::new(2, 2);

        fb_b.set(0, 0, Cell::Filled { character: 'P' });
        fb_b.set(0, 1, Cell::Filled { character: 'H' });

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
        fb_a.set(0, 0, Cell::Filled { character: 'P' });
        fb_a.set(1, 1, Cell::Filled { character: 'o' }); // remove

        let mut fb_b = Framebuffer::new(2, 6);
        fb_b.set(0, 0, Cell::Filled { character: '!' }); // up
        fb_b.set(1, 3, Cell::Filled { character: '1' }); // add

        println!("fb back a {:?}", fb_a.buf);
        println!("fb back b {:?}", fb_b.buf);

        let diff = compare(&fb_a, &fb_b);

        // diff.iter().for_each(|d| println!("{:?}", d));
        assert_eq!(diff.len(), 3);

        // (0, 0), (1, 0),
        // (0, 1), (1, 1),
        // (0, 2), (1, 2),

        // (0, 0), (1, 0)
        // (0, 1), (1, 1)
        // (0, 2), (1, 2)

        // Note(ph): I believe there is an issue with the iterator implementation or set/get.
        // Compare the two inner vec buffers.
    }
}
