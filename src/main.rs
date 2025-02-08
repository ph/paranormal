mod framebuffer;
mod terminal;
mod windows;

use framebuffer::{Cell, Framebuffer};
use terminal::{window_size, Color, Command, Style};

fn main() {
    let mut fb = Framebuffer::new(10, 20);

    fb.set(2, 3, Cell::Filled { character: 'P' });
    fb.set(2, 4, Cell::Filled { character: 'H' });
    fb.set(2, 5, Cell::Filled { character: '!' });

    // use Command::*;
    // use Style::*;

    // let a = vec![
    //     Clear,
    //     MoveTo(2, 2),
    //     ApplyStyle(Background(Color::Red)),
    //     ApplyStyle(Foreground(Color::Yellow)),
    //     Write("Wooo"),
    //     ApplyStyle(Reset),
    //     ApplyStyle(Foreground(Color::Cyan)),
    //     Write(" YESt "),
    // ];

    // for i in a {
    //     print!("{}", i);
    // }

    let w = window_size();
    println!("window size: {:?}", w);
}
