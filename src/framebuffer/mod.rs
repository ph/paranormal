mod diff;

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
        assert!(x < self.width, "X is out of bound of {}", self.width);
        assert!(y < self.height, "Y is out of bounf of {}", self.height);

        self.buf.insert(self.idx(x, y), cell)
    }

    pub fn get(&self, x: u16, y: u16) -> &Cell {
        assert!(x < self.width, "X is out of bound of {}", self.width);
        assert!(y < self.height, "Y is out of bounf of {}", self.height);

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
    x: u16,
    y: u16,
}

impl<'a> FramebufferIterator<'a> {
    fn new(fb: &'a Framebuffer) -> Self {
        Self { fb: fb, x: 0, y: 0 }
    }
}

impl<'a> Iterator for FramebufferIterator<'a> {
    type Item = ((u16, u16), &'a Cell);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.fb.width() {
            self.y = 0;
            self.x += 1;
        }

        if self.x >= self.fb.height() {
            return None;
        }

        let cell = ((self.x, self.y), self.fb.get(self.x, self.y));

        self.y += 1;

        Some(cell)
    }
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
            ((0, 1), &Cell::Empty),
            ((1, 0), &Cell::Empty),
            ((1, 1), &cell_2),
        ];

        assert_eq!(fb.iter().collect::<Vec<_>>(), expected);
    }
}
