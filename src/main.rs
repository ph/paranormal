mod framebuffer;
mod renderer;
mod terminal;

use framebuffer::{diff, Cell, Framebuffer};
use renderer::{Renderer, Terminal};
use terminal::{bg, fg, window_size, Color, Command, Style};

extern crate libc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let w = window_size()?;
    println!("window size: {:?}", w);

    let fb = Framebuffer::new(w.cols, w.rows);
    let mut fb_1 = fb.clone();

    fb_1.set(
        2,
        3,
        Cell::Filled {
            character: 'P',
            foreground: fg(Color::Magenta),
            background: bg(Color::Cyan),
        },
    );
    fb_1.set(
        2,
        4,
        Cell::Filled {
            character: 'H',
            foreground: fg(Color::Red),
            background: bg(Color::Black),
        },
    );
    fb_1.set(
        2,
        5,
        Cell::Filled {
            character: '!',
            foreground: fg(Color::White),
            background: bg(Color::Yellow),
        },
    );

    let changesets = diff::compare(&fb, &fb_1);
    println!("changesets:\n {:?}", changesets);
    let mut out = std::io::stdout();
    let _ = terminal::configure(&mut out);
    let mut renderer = Terminal::new(&mut out);
    renderer.submit(changesets);

    Ok(())
}
