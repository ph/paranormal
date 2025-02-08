use std::{
    borrow::Cow,
    io::{Error, Stdout, Write},
    os::fd::{AsRawFd, IntoRawFd},
};

use libc::{TIOCGWINSZ, ioctl, winsize};

static TTY: &str = "/dev/tty";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Style {
    Foreground(Color),
    Background(Color),
    Reset,
}

impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Style::*;
        match self {
            Foreground(color) => match color {
                Color::Rgb { .. } => write!(f, "\x1B[38;2{}m", color.fg()),
                _ => write!(f, "\x1B[38;{}m", color.fg()),
            },

            Background(color) => match color {
                Color::Rgb { .. } => write!(f, "\x1B[48;2{}m", color.fg()),
                _ => write!(f, "\x1B[48;{}m", color.bg()),
            },
            Reset => write!(f, "\x1B[0m"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Visibility {
    Hidden,
    Show,
}

#[derive(Debug, Clone)]
pub enum Command {
    MoveTo(u16, u16),
    ApplyStyle(Style),
    Write(String),
    Cursor(Visibility),
    Clear,
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Command::MoveTo(x, y) => write!(f, "\x1B[{};{}H", x, y),
            Command::ApplyStyle(style) => write!(f, "{}", style),
            Command::Write(s) => write!(f, "{}", s),
            Command::Cursor(visibility) => match visibility {
                Visibility::Hidden => write!(f, "{}", "\x1B[?25l"),
                Visibility::Show => write!(f, "{}", "\x1B[?25h"),
            },
            Command::Clear => write!(f, "{}", "\x1B[2J"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Color {
    Rgb { r: u8, g: u8, b: u8 },

    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl Color {
    pub fn bg(&self) -> Cow<str> {
        use Color::*;
        match self {
            Rgb { r, g, b } => Cow::Owned(format!("{};{};{}", r, g, b)),
            Black => Cow::Borrowed("40"),
            Red => Cow::Borrowed("41"),
            Green => Cow::Borrowed("42"),
            Yellow => Cow::Borrowed("43"),
            Blue => Cow::Borrowed("44"),
            Magenta => Cow::Borrowed("45"),
            Cyan => Cow::Borrowed("46"),
            White => Cow::Borrowed("47"),
            BrightBlack => Cow::Borrowed("100"),
            BrightRed => Cow::Borrowed("101"),
            BrightGreen => Cow::Borrowed("102"),
            BrightYellow => Cow::Borrowed("103"),
            BrightBlue => Cow::Borrowed("104"),
            BrightMagenta => Cow::Borrowed("105"),
            BrightCyan => Cow::Borrowed("106"),
            BrightWhite => Cow::Borrowed("107"),
        }
    }

    pub fn fg(&self) -> Cow<str> {
        use Color::*;
        match self {
            Rgb { r, g, b } => Cow::Owned(format!("{};{};{}", r, g, b)),
            Black => Cow::Borrowed("30"),
            Red => Cow::Borrowed("31"),
            Green => Cow::Borrowed("32"),
            Yellow => Cow::Borrowed("33"),
            Blue => Cow::Borrowed("34"),
            Magenta => Cow::Borrowed("35"),
            Cyan => Cow::Borrowed("36"),
            White => Cow::Borrowed("37"),
            BrightBlack => Cow::Borrowed("90"),
            BrightRed => Cow::Borrowed("91"),
            BrightGreen => Cow::Borrowed("92"),
            BrightYellow => Cow::Borrowed("93"),
            BrightBlue => Cow::Borrowed("94"),
            BrightMagenta => Cow::Borrowed("95"),
            BrightCyan => Cow::Borrowed("96"),
            BrightWhite => Cow::Borrowed("97"),
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct WinSize {
    pub rows: u16,
    pub cols: u16,
    pub x_pixel: u16,
    pub y_pixel: u16,
}

pub fn window_size() -> Result<WinSize, Error> {
    let fd = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(TTY)
        .map(|f| f.into_raw_fd())?;

    let w = window_size_from(fd)?;

    unsafe {
        if libc::close(fd) < 0 {
            return Err(Error::last_os_error());
        }
    };

    Ok(w)
}

pub fn window_size_from(fd: i32) -> Result<WinSize, Error> {
    let mut w = WinSize {
        rows: 0,
        cols: 0,
        x_pixel: 0,
        y_pixel: 0,
    };

    unsafe {
        if ioctl(fd, TIOCGWINSZ, &mut w) < 0 {
            return Err(Error::last_os_error());
        }
    }

    Ok(w.into())
}

pub fn bg(c: Color) -> Style {
    Style::Background(c)
}

pub fn fg(c: Color) -> Style {
    Style::Foreground(c)
}

pub fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::Rgb { r: r, g: g, b: b }
}

pub fn configure<W: Write>(out: &mut W) -> std::io::Result<()> {
    write!(out, "{}", Command::Clear)?;
    Ok(())
}
