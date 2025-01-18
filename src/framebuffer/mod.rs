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

struct FramebufferIterator<'a> {
    fb: &'a Framebuffer,
}

impl<'a> FramebufferIterator<'a> {
    fn new(fb: &'a Framebuffer) -> Self {
        Self { fb }
    }
}

impl<'a> Iterator for FramebufferIterator<'a> {
    type Item = ((u16, u16), &'a Cell);

    fn next(&mut self) -> Option<Self::Item> {
        for x in 0..self.fb.width() {
            for y in 0..self.fb.height() {
                let cell = self.fb.get(x, y);
                return Some(((x, y), cell));
            }
        }
        None
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
}
