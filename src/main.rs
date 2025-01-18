use framebuffer::{Cell, Framebuffer};

mod framebuffer;

fn main() {
    let mut fb = Framebuffer::new(10, 20);

    fb.set(2, 3, Cell::Filled { character: 'P' });
    fb.set(2, 4, Cell::Filled { character: 'H' });
    fb.set(2, 5, Cell::Filled { character: '!' });
}
