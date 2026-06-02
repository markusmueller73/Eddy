// Part of Eddy - A lightweight text editor for the terminal.
use crossterm::style::{Color, Colors, available_color_count};

pub struct ColorPairs {
    bg: Color,
    fg: Color,
    bg_hilite: Color,
    fg_hilite: Color,
    bg_bar: Color,
    fg_bar: Color,
    bg_linenb: Color,
    fg_linenb: Color,
    fg_marker: Color,
}

#[allow(dead_code)]
impl ColorPairs {
    pub fn new() -> ColorPairs {
        let nb_of_colors = available_color_count();
        debug!("<ColorPairs::new>: nb_of_colors={}", nb_of_colors);
        let cp: ColorPairs = match nb_of_colors {
            0..=2 => {
                ColorPairs {
                    bg: Color::Black,
                    fg: Color::White,
                    bg_hilite: Color::White,
                    fg_hilite: Color::Black,
                    bg_bar: Color::White,
                    fg_bar: Color::Black,
                    bg_linenb: Color::Black,
                    fg_linenb: Color::White,
                    fg_marker: Color::White,
                }
            },
            3..=8 => {
                ColorPairs {
                    bg: Color::Black,
                    fg: Color::White,
                    bg_hilite: Color::Grey,
                    fg_hilite: Color::White,
                    bg_bar: Color::White,
                    fg_bar: Color::Black,
                    bg_linenb: Color::Grey,
                    fg_linenb: Color::White,
                    fg_marker: Color::Green,
                }
            },
            9..=16 => {
                ColorPairs {
                    bg: Color::Black,
                    fg: Color::White,
                    bg_hilite: Color::DarkGrey,
                    fg_hilite: Color::White,
                    bg_bar: Color::White,
                    fg_bar: Color::Black,
                    bg_linenb: Color::Grey,
                    fg_linenb: Color::White,
                    fg_marker: Color::Green,
                }
            },
            17..=255 => {
                ColorPairs {
                    bg: Color::AnsiValue(0),
                    fg: Color::AnsiValue(7),
                    bg_hilite: Color::AnsiValue(241),
                    fg_hilite: Color::AnsiValue(231),
                    bg_bar: Color::AnsiValue(253),
                    fg_bar: Color::AnsiValue(235),
                    bg_linenb: Color::AnsiValue(239),
                    fg_linenb: Color::AnsiValue(249),
                    fg_marker: Color::AnsiValue(34),
                }
            },
            256.. => {
                ColorPairs {
                    bg: Color::Rgb {r: 0, g: 0, b: 0},
                    fg: Color::Rgb {r: 224, g: 224, b: 224},
                    bg_hilite: Color::Rgb {r: 48, g: 48, b: 48},
                    fg_hilite: Color::Rgb {r: 224, g: 224, b: 224},
                    bg_bar: Color::Rgb {r: 192, g: 192, b: 192},
                    fg_bar: Color::Rgb {r: 32, g: 32, b: 32},
                    bg_linenb: Color::Rgb {r: 32, g: 32, b: 32},
                    fg_linenb: Color::Rgb {r: 160, g: 160, b: 160},
                    fg_marker: Color::Rgb {r: 0, g: 192, b: 0},
                }
            }
        };
        cp
    }
    pub fn default_pair(&self) -> Colors {
        Colors::new(self.fg, self.bg)
    }
    pub fn hilite_pair(&self) -> Colors {
        Colors::new(self.fg_hilite, self.bg_hilite)
    }
    pub fn bar_pair(&self) -> Colors {
        Colors::new(self.fg_bar, self.bg_bar)
    }
    pub fn linenb_pair(&self) -> Colors {
        Colors::new(self.fg_linenb, self.bg_linenb)
    }
    pub fn marker_pair(&self) -> Colors {
        Colors::new(self.bg_linenb, self.fg_linenb)
    }
    pub fn special_pair(&self) -> Colors {
        Colors::new(self.fg_marker, self.bg)
    }
}
