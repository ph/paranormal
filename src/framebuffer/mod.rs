use std::fmt::Write;

mod diff;

#[derive(Debug, Clone)]
pub enum FramebufferErr {
    Writing(String),
}

impl std::error::Error for FramebufferErr {}

impl std::fmt::Display for FramebufferErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FramebufferErr::Writing(err) => write!(f, "can't write to buffer, error: {}", err),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Filled { character: char },
}

#[derive(Debug, Clone)]
pub struct Framebuffer {
    width: u16,
    height: u16,
    buf: Vec<Cell>,
}

impl Framebuffer {
    pub fn new(width: u16, height: u16) -> Self {
        const EMPTY: Cell = Cell::Empty;

        let capacity = width * height;
        let buf = vec![EMPTY; capacity.into()];

        Self { width, height, buf }
    }

    fn idx(&self, x: u16, y: u16) -> usize {
        (x * self.width + y).into()
    }

    pub fn set(&mut self, x: u16, y: u16, cell: Cell) {
        assert!(
            x < self.width,
            "X value of {} is out of bound of {}",
            x,
            self.width
        );
        assert!(
            y < self.height,
            "Y value of {} is out of bounf of {}",
            y,
            self.height
        );

        self.buf.insert(self.idx(x, y), cell)
    }

    pub fn get(&self, x: u16, y: u16) -> &Cell {
        assert!(
            x < self.width,
            "X value of {} is out of bound of {}",
            x,
            self.width
        );
        assert!(
            y < self.height,
            "Y value of {} is out of bounf of {}",
            y,
            self.height
        );

        match self.buf.get(self.idx(x, y)) {
            Some(cell) => cell,
            None => &Cell::Empty,
        }
    }

    #[allow(unused)]
    pub fn resize(&mut self, _x: u16, _y: u16) {
        unimplemented!()
    }

    #[allow(unused)]
    pub fn clear(&mut self) {
        self.buf.clear()
    }

    #[allow(unused)]
    pub fn capacity(&self) -> usize {
        (self.width * self.height).into()
    }

    #[allow(unused)]
    pub fn width(&self) -> u16 {
        self.width
    }

    #[allow(unused)]
    pub fn height(&self) -> u16 {
        self.height
    }

    pub fn iter(&self) -> FramebufferIterator {
        FramebufferIterator::new(self)
    }
}

pub struct FramebufferIterator<'a> {
    fb: &'a Framebuffer,
    idx: usize,
}

impl<'a> FramebufferIterator<'a> {
    fn new(fb: &'a Framebuffer) -> Self {
        Self { fb: fb, idx: 0 }
    }
}

impl<'a> Iterator for FramebufferIterator<'a> {
    type Item = ((u16, u16), &'a Cell);

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == self.fb.capacity() {
            return None;
        }

        let x: u16 = (self.idx % self.fb.width() as usize)
            .try_into()
            .expect("X position is out of bound");

        let y: u16 = (self.idx / self.fb.width() as usize)
            .try_into()
            .expect(" Y position is out of bound");

        let cell = ((x, y), self.fb.get(x, y));

        self.idx += 1;

        Some(cell)
    }
}

pub fn render<W: Write>(fb: &Framebuffer, out: &mut W) -> Result<(), FramebufferErr> {
    let mut i = 0;

    for ((_x, _y), cell) in fb.iter() {
        i += 1;

        match cell {
            Cell::Empty => write!(out, " ").map_err(|e| FramebufferErr::Writing(e.to_string()))?,
            Cell::Filled { character } => {
                write!(out, "{}", character).map_err(|e| FramebufferErr::Writing(e.to_string()))?
            }
        }

        if i % fb.width() == 0 {
            write!(out, "\n").map_err(|e| FramebufferErr::Writing(e.to_string()))?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn set_and_get() {
        let mut fb = Framebuffer::new(3, 4);
        let cell = Cell::Filled { character: 'a' };
        fb.set(1, 1, cell.clone());
        let c = fb.get(1, 1);
        assert_eq!(cell, *c);
    }

    #[test]
    fn capacity() {
        let fb = Framebuffer::new(3, 4);
        assert_eq!(fb.capacity(), 12);
    }

    #[test]
    fn clear() {
        let mut fb = Framebuffer::new(3, 4);
        let cell = Cell::Filled { character: 'a' };
        fb.set(1, 1, cell.clone());
        let c = fb.get(1, 1);
        assert_eq!(cell, *c);

        fb.clear();
        assert_eq!(*fb.get(1, 2), Cell::Empty);
    }

    #[test]
    fn height() {
        let fb = Framebuffer::new(3, 4);
        assert_eq!(fb.height(), 4);
    }

    #[test]
    fn width() {
        let fb = Framebuffer::new(3, 4);
        assert_eq!(fb.width(), 3);
    }

    #[test]
    fn iterator() {
        let mut fb = Framebuffer::new(2, 2);

        let cell_1 = Cell::Filled { character: 'X' };
        let cell_2 = Cell::Filled { character: 'Y' };

        fb.set(0, 0, cell_1.clone());
        fb.set(1, 1, cell_2.clone());

        let expected = vec![
            ((0, 0), &cell_1),
            ((1, 0), &Cell::Empty),
            ((0, 1), &Cell::Empty),
            ((1, 1), &cell_2),
        ];

        assert_eq!(fb.iter().collect::<Vec<_>>(), expected);
    }

    #[test]
    fn render_to_raw_buffer() {
        let mut fb = Framebuffer::new(2, 2);
        fb.set(0, 0, Cell::Filled { character: 'X' });
        fb.set(1, 1, Cell::Filled { character: 'Y' });

        let mut buf = String::new();

        render(&fb, &mut buf).expect("should be able to write to the buffer");

        assert_eq!(buf, String::from("X \n Y\n"));
    }
}
