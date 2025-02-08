mod framebuffer;
mod renderer;
mod terminal;
mod windows;

use framebuffer::{diff, Cell, Framebuffer};
use renderer::{Renderer, Terminal};
use terminal::{bg, fg, window_size, Color, Command, Style};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let w = window_size()?;
    println!("window size: {:?}", w);

    let mut fb = Framebuffer::new(w.cols, w.rows);
    let mut fb_1 = fb.clone();

    use Color::*;

    fb_1.set(
        2,
        3,
        Cell::Filled {
            character: 'P',
            foreground: fg(Magenta),
            background: bg(Cyan),
        },
    );
    fb_1.set(
        2,
        4,
        Cell::Filled {
            character: 'H',
            foreground: fg(Red),
            background: bg(Black),
        },
    );
    fb_1.set(
        2,
        5,
        Cell::Filled {
            character: '!',
            foreground: fg(White),
            background: bg(Yellow),
        },
    );

    let changesets = diff::compare(&fb, &fb_1);
    println!("changesets:\n {:?}", changesets);
    let mut out = std::io::stdout();
    terminal::configure(&mut out);
    let mut renderer = Terminal::new(&mut out);
    renderer.submit(changesets);

    Ok(())
}
