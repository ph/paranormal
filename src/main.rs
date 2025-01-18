use framebuffer::{Cell, Framebuffer};

mod framebuffer;

fn main() {
    let mut fb = Framebuffer::new(10, 20);

    fb.set(2, 3, Cell::new('P'));
    fb.set(2, 4, Cell::new('H'));
    fb.set(2, 5, Cell::new('!'));
}
