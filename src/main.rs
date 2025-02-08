mod framebuffer;
mod terminal;

use framebuffer::{Cell, Framebuffer};
use terminal::{Color, Command, Style};

fn main() {
    let mut fb = Framebuffer::new(10, 20);

    fb.set(2, 3, Cell::Filled { character: 'P' });
    fb.set(2, 4, Cell::Filled { character: 'H' });
    fb.set(2, 5, Cell::Filled { character: '!' });

    use Command::*;
    use Style::*;

    let a = vec![
        Clear,
        MoveTo(2, 2),
        ApplyStyle(Background(Color::Red)),
        ApplyStyle(Foreground(Color::Yellow)),
        Write("Wooo"),
        ApplyStyle(Reset),
        ApplyStyle(Foreground(Color::Cyan)),
        Write(" YES"),
    ];

    for i in a {
        print!("{}", i);
    }
}
