#[derive(Debug, Clone)]
pub enum Style {
    Foreground(Color),
    Background(Color),
}

impl std::fmt::Display for Style {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Style::*;
        match self {
            Foreground(color) => match color {
                Color::Rgb { .. } => write!(f, "\x1B[38;2{}m", color.fg()),
                _ => write!(f, "\x1B[38:5{}m]"),
            },

            Background(color) => match color {
                Color::Rgb { .. } => write!(f, "\x1B[48;2{}m", color.fg()),
                _ => write!(f, "\x1B[48:5{}m]"),
            },
        }
    }
}

#[derive(Debug, Clone)]
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
    pub fn bg(&self) -> &'static str {
        use Color::*;
        match self {
            Rgb { r, g, b } => &format!("{};{};{}", r, g, b),
            Black => "40",
            Red => "41",
            Green => "42",
            Yellow => "43",
            Blue => "44",
            Magenta => "45",
            Cyan => "46",
            White => "47",
            BrightBlack => "100",
            BrightRed => "101",
            BrightGreen => "102",
            BrightYellow => "103",
            BrightBlue => "104",
            BrightMagenta => "105",
            BrightCyan => "106",
            BrightWhite => "107",
        }
    }

    pub fn fg(&self) -> &'static str {
        use Color::*;
        match self {
            Rgb { r, g, b } => &format!("{};{};{}", r, g, b),
            Black => "30",
            Red => "31",
            Green => "32",
            Yellow => "33",
            Blue => "34",
            Magenta => "35",
            Cyan => "36",
            White => "37",
            BrightBlack => "90",
            BrightRed => "91",
            BrightGreen => "92",
            BrightYellow => "93",
            BrightBlue => "94",
            BrightMagenta => "95",
            BrightCyan => "96",
            BrightWhite => "97",
        }
    }
}
