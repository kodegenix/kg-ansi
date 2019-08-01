use std::fmt::Write;
use std::ops::{Deref, DerefMut};


const EFFECT_BOLD: u8 = 0x01;
const EFFECT_DIMMED: u8 = 0x02;
const EFFECT_ITALIC: u8 = 0x04;
const EFFECT_UNDERLINE: u8 = 0x08;
const EFFECT_BLINK: u8 = 0x10;
const EFFECT_INVERT: u8 = 0x20;
const EFFECT_HIDDEN: u8 = 0x40;
const EFFECT_STRIKETROUGH: u8 = 0x80;


static EFFECTS: [&'static str; 256] = [
    "", "1", "2", "1;2", "3", "1;3", "2;3", "1;2;3",
    "4", "1;4", "2;4", "1;2;4", "3;4", "1;3;4", "2;3;4", "1;2;3;4",
    "5", "1;5", "2;5", "1;2;5", "3;5", "1;3;5", "2;3;5", "1;2;3;5",
    "4;5", "1;4;5", "2;4;5", "1;2;4;5", "3;4;5", "1;3;4;5", "2;3;4;5", "1;2;3;4;5",
    "7", "1;7", "2;7", "1;2;7", "3;7", "1;3;7", "2;3;7", "1;2;3;7",
    "4;7", "1;4;7", "2;4;7", "1;2;4;7", "3;4;7", "1;3;4;7", "2;3;4;7", "1;2;3;4;7",
    "5;7", "1;5;7", "2;5;7", "1;2;5;7", "3;5;7", "1;3;5;7", "2;3;5;7", "1;2;3;5;7",
    "4;5;7", "1;4;5;7", "2;4;5;7", "1;2;4;5;7", "3;4;5;7", "1;3;4;5;7", "2;3;4;5;7", "1;2;3;4;5;7",
    "8", "1;8", "2;8", "1;2;8", "3;8", "1;3;8", "2;3;8", "1;2;3;8",
    "4;8", "1;4;8", "2;4;8", "1;2;4;8", "3;4;8", "1;3;4;8", "2;3;4;8", "1;2;3;4;8",
    "5;8", "1;5;8", "2;5;8", "1;2;5;8", "3;5;8", "1;3;5;8", "2;3;5;8", "1;2;3;5;8",
    "4;5;8", "1;4;5;8", "2;4;5;8", "1;2;4;5;8", "3;4;5;8", "1;3;4;5;8", "2;3;4;5;8", "1;2;3;4;5;8",
    "7;8", "1;7;8", "2;7;8", "1;2;7;8", "3;7;8", "1;3;7;8", "2;3;7;8", "1;2;3;7;8",
    "4;7;8", "1;4;7;8", "2;4;7;8", "1;2;4;7;8", "3;4;7;8", "1;3;4;7;8", "2;3;4;7;8", "1;2;3;4;7;8",
    "5;7;8", "1;5;7;8", "2;5;7;8", "1;2;5;7;8", "3;5;7;8", "1;3;5;7;8", "2;3;5;7;8", "1;2;3;5;7;8",
    "4;5;7;8", "1;4;5;7;8", "2;4;5;7;8", "1;2;4;5;7;8", "3;4;5;7;8", "1;3;4;5;7;8", "2;3;4;5;7;8", "1;2;3;4;5;7;8",
    "9", "1;9", "2;9", "1;2;9", "3;9", "1;3;9", "2;3;9", "1;2;3;9",
    "4;9", "1;4;9", "2;4;9", "1;2;4;9", "3;4;9", "1;3;4;9", "2;3;4;9", "1;2;3;4;9",
    "5;9", "1;5;9", "2;5;9", "1;2;5;9", "3;5;9", "1;3;5;9", "2;3;5;9", "1;2;3;5;9",
    "4;5;9", "1;4;5;9", "2;4;5;9", "1;2;4;5;9", "3;4;5;9", "1;3;4;5;9", "2;3;4;5;9", "1;2;3;4;5;9",
    "7;9", "1;7;9", "2;7;9", "1;2;7;9", "3;7;9", "1;3;7;9", "2;3;7;9", "1;2;3;7;9",
    "4;7;9", "1;4;7;9", "2;4;7;9", "1;2;4;7;9", "3;4;7;9", "1;3;4;7;9", "2;3;4;7;9", "1;2;3;4;7;9",
    "5;7;9", "1;5;7;9", "2;5;7;9", "1;2;5;7;9", "3;5;7;9", "1;3;5;7;9", "2;3;5;7;9", "1;2;3;5;7;9",
    "4;5;7;9", "1;4;5;7;9", "2;4;5;7;9", "1;2;4;5;7;9", "3;4;5;7;9", "1;3;4;5;7;9", "2;3;4;5;7;9", "1;2;3;4;5;7;9",
    "8;9", "1;8;9", "2;8;9", "1;2;8;9", "3;8;9", "1;3;8;9", "2;3;8;9", "1;2;3;8;9",
    "4;8;9", "1;4;8;9", "2;4;8;9", "1;2;4;8;9", "3;4;8;9", "1;3;4;8;9", "2;3;4;8;9", "1;2;3;4;8;9",
    "5;8;9", "1;5;8;9", "2;5;8;9", "1;2;5;8;9", "3;5;8;9", "1;3;5;8;9", "2;3;5;8;9", "1;2;3;5;8;9",
    "4;5;8;9", "1;4;5;8;9", "2;4;5;8;9", "1;2;4;5;8;9", "3;4;5;8;9", "1;3;4;5;8;9", "2;3;4;5;8;9", "1;2;3;4;5;8;9",
    "7;8;9", "1;7;8;9", "2;7;8;9", "1;2;7;8;9", "3;7;8;9", "1;3;7;8;9", "2;3;7;8;9", "1;2;3;7;8;9",
    "4;7;8;9", "1;4;7;8;9", "2;4;7;8;9", "1;2;4;7;8;9", "3;4;7;8;9", "1;3;4;7;8;9", "2;3;4;7;8;9", "1;2;3;4;7;8;9",
    "5;7;8;9", "1;5;7;8;9", "2;5;7;8;9", "1;2;5;7;8;9", "3;5;7;8;9", "1;3;5;7;8;9", "2;3;5;7;8;9", "1;2;3;5;7;8;9",
    "4;5;7;8;9", "1;4;5;7;8;9", "2;4;5;7;8;9", "1;2;4;5;7;8;9", "3;4;5;7;8;9", "1;3;4;5;7;8;9", "2;3;4;5;7;8;9", "1;2;3;4;5;7;8;9",
];


struct SepPrinter<'a, 'b: 'a> {
    sep: bool,
    fmt: &'a mut std::fmt::Formatter<'b>,
}

impl<'a, 'b> SepPrinter<'a, 'b> {
    fn new(fmt: &'a mut std::fmt::Formatter<'b>) -> SepPrinter<'a, 'b> {
        SepPrinter {
            sep: false,
            fmt,
        }
    }
}

impl<'a, 'b> std::fmt::Write for SepPrinter<'a, 'b> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        if self.sep {
            self.fmt.write_char(';')?;
        }
        if !s.is_empty() {
            self.sep = true;
            self.fmt.write_str(s)?;
        }
        Ok(())
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Effect(u8);

impl Effect {
    pub fn new() -> Effect {
        Effect(0)
    }

    pub fn bold(self) -> Effect {
        Effect(self.0 | EFFECT_BOLD)
    }

    pub fn dimmed(self) -> Effect {
        Effect(self.0 | EFFECT_DIMMED)
    }

    pub fn italic(self) -> Effect {
        Effect(self.0 | EFFECT_ITALIC)
    }

    pub fn underline(self) -> Effect {
        Effect(self.0 | EFFECT_UNDERLINE)
    }

    pub fn blink(self) -> Effect {
        Effect(self.0 | EFFECT_BLINK)
    }

    pub fn invert(self) -> Effect {
        Effect(self.0 | EFFECT_INVERT)
    }

    pub fn hidden(self) -> Effect {
        Effect(self.0 | EFFECT_HIDDEN)
    }

    pub fn striketrough(self) -> Effect {
        Effect(self.0 | EFFECT_STRIKETROUGH)
    }

    fn write(&self, p: &mut SepPrinter) -> std::fmt::Result {
        write!(p, "{}", EFFECTS[self.0 as usize])
    }
}


impl std::default::Default for Effect {
    fn default() -> Self {
        Effect::new()
    }
}


#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Color {
    None,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White,
    Index8(u8),
    Rgb(u8, u8, u8),
}

impl Color {
    fn write_fg(&self, p: &mut SepPrinter) -> std::fmt::Result {
        match *self {
            Color::None => Ok(()),
            Color::Black => write!(p, "30"),
            Color::Red => write!(p, "31"),
            Color::Green => write!(p, "32"),
            Color::Yellow => write!(p, "33"),
            Color::Blue => write!(p, "34"),
            Color::Purple => write!(p, "35"),
            Color::Cyan => write!(p, "36"),
            Color::White => write!(p, "37"),
            Color::Index8(num) => write!(p, "38;5;{}", num),
            Color::Rgb(r, g, b) => write!(p, "38;2;{};{};{}", r, g, b)
        }
    }

    fn write_bg(&self, p: &mut SepPrinter) -> std::fmt::Result {
        match *self {
            Color::None => Ok(()),
            Color::Black => write!(p, "40"),
            Color::Red => write!(p, "41"),
            Color::Green => write!(p, "42"),
            Color::Yellow => write!(p, "43"),
            Color::Blue => write!(p, "44"),
            Color::Purple => write!(p, "45"),
            Color::Cyan => write!(p, "46"),
            Color::White => write!(p, "47"),
            Color::Index8(num) => write!(p, "48;5;{}", num),
            Color::Rgb(r, g, b) => write!(p, "48;2;{};{};{}", r, g, b)
        }
    }
}

impl std::default::Default for Color {
    fn default() -> Self {
        Color::None
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Style {
    effect: Effect,
    fg: Color,
    bg: Color,
}

impl Style {
    pub fn bold(self) -> Self {
        Style {
            effect: self.effect.bold(),
            ..self
        }
    }

    pub fn underline(self) -> Self {
        Style {
            effect: self.effect.underline(),
            ..self
        }
    }

}

impl std::default::Default for Style {
    fn default() -> Self {
        Style {
            effect: Effect::default(),
            fg: Color::default(),
            bg: Color::default(),
        }
    }
}


pub struct Colored<'a, T: 'static> {
    item: &'a T,
    style: Style,
}

impl<'a, T> Colored<'a, T> {
    pub fn bold(self) -> Colored<'a, T> {
        Colored {
            item: self.item,
            style: self.style.bold(),
        }
    }

    pub fn underline(self) -> Colored<'a, T> {
        Colored {
            item: self.item,
            style: self.style.underline(),
        }
    }
}


pub trait Colorable<T> {
    fn bold(&self) -> Colored<T>;

    fn underline(&self) -> Colored<T>;
}

impl<T: std::fmt::Display> Colorable<T> for T {
    fn bold(&self) -> Colored<T> {
        Colored {
            item: self,
            style: Style {
                effect: Effect::new().bold(),
                ..Style::default()
            },
        }
    }

    fn underline(&self) -> Colored<T> {
        Colored {
            item: self,
            style: Style {
                effect: Effect::new().underline(),
                fg: Color::White,
                ..Style::default()
            },
        }
    }
}


pub struct AnsiFormatter<'a: 'b, 'b> {
    f: &'b mut std::fmt::Formatter<'a>,
    style: Vec<Style>,
}

impl<'a, 'b> AnsiFormatter<'a, 'b> {
    fn write_escape(&mut self, e: Style) -> AnsiResult {
        write!(self.f, "\x1B[0m\x1B[")?;
        {
            let mut p = SepPrinter::new(self.f);
            e.effect.write(&mut p)?;
            if e.fg != Color::None {
                e.fg.write_fg(&mut p)?;
            }
            if e.bg != Color::None {
                e.bg.write_bg(&mut p)?;
            }
        }
        write!(self.f, "m")?;
        Ok(())
    }

    fn write_reset(&mut self) -> AnsiResult {
        write!(self.f, "\x1B[0m")
    }

    fn refresh(&mut self) -> AnsiResult {
        let e = self.style.last().cloned();
        match e {
            Some(e) => self.write_escape(e),
            None => self.write_reset(),
        }
    }

    pub fn push(&mut self, e: Style) -> AnsiResult {
        self.style.push(e);
        self.refresh()
    }

    pub fn pop(&mut self) -> AnsiResult {
        self.style.pop();
        self.refresh()
    }
}

impl<'a, 'b> Deref for AnsiFormatter<'a, 'b> {
    type Target = std::fmt::Formatter<'a>;

    fn deref(&self) -> &Self::Target {
        self.f
    }
}

impl<'a, 'b> DerefMut for AnsiFormatter<'a, 'b> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.f
    }
}


pub type AnsiResult = std::fmt::Result;

pub trait AnsiDisplay {
    fn fmt(&self, formatter: &mut AnsiFormatter) -> AnsiResult;
}

pub trait ToAnsiString {
    fn to_ansi_string(&self) -> String;
}

struct AnsiDisplayAdapter<'a, T: AnsiDisplay + 'static>(&'a T);

impl<'a, T: AnsiDisplay + 'static> std::fmt::Display for AnsiDisplayAdapter<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut fm = AnsiFormatter {
            f,
            style: Vec::new(),
        };
        self.0.fmt(&mut fm)
    }
}

impl<T: AnsiDisplay + 'static> ToAnsiString for T {
    fn to_ansi_string(&self) -> String {
        let mut s = String::new();
        s.write_fmt(format_args!("{}", AnsiDisplayAdapter(self)))
            .expect("a Display implementation return an error unexpectedly");
        s.shrink_to_fit();
        s
    }
}

impl<'a, T: std::fmt::Display> AnsiDisplay for Colored<'a, T> {
    fn fmt(&self, f: &mut AnsiFormatter) -> AnsiResult {
        f.push(self.style)?;
        self.item.fmt(f.f)?;
        f.pop()?;
        Ok(())
    }
}


impl<'a, T: std::fmt::Display> std::fmt::Display for Colored<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.item.fmt(f)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn style() {
        struct Point {
            x: f64,
            y: f64,
        }

        impl AnsiDisplay for Point {
            fn fmt(&self, f: &mut AnsiFormatter) -> AnsiResult {
                let ef = Style {
                    effect: Effect::new().bold(),
                    fg: Color::Green,
                    bg: Color::None,
                };
                let eop = Style {
                    effect: Effect::new(),
                    fg: Color::Yellow,
                    bg: Color::None,
                };
                let ev = Style {
                    effect: Effect::new().bold().underline(),
                    fg: Color::Blue,
                    bg: Color::None,
                };

                f.push(ef)?;
                write!(f, "x")?;
                f.push(eop)?;
                write!(f, ": ")?;
                f.pop()?;
                f.push(ev)?;
                write!(f, "{}", self.x)?;
                f.pop()?;
                write!(f, " km ")?;
                f.pop()?;

                f.push(ef)?;
                write!(f, "y")?;
                f.push(eop)?;
                write!(f, ": ")?;
                f.pop()?;
                f.push(ev)?;
                write!(f, "{}", self.y)?;
                f.pop()?;
                write!(f, " km")?;
                f.pop()?;
                Ok(())
            }
        }

        struct Rectangle {
            a: Point,
            b: Point,
        }

        impl AnsiDisplay for Rectangle {
            fn fmt(&self, f: &mut AnsiFormatter) -> AnsiResult {
                let ef = Style {
                    effect: Effect::new().bold(),
                    fg: Color::Red,
                    bg: Color::None,
                };

                f.push(ef)?;
                writeln!(f, "Rectangle {{")?;
                write!(f, "  a = ")?;
                self.a.fmt(f)?;
                write!(f, "\n  b = ")?;
                self.b.fmt(f)?;
                writeln!(f, "\n}}")?;
                f.pop()?;
                Ok(())
            }
        }

        println!("{}", "testing".underline().to_ansi_string());

        let r = Rectangle {
            a: Point { x: 10.5, y: 12.4 },
            b: Point { x: -2., y: -13. },
        };

        println!("{}", r.to_ansi_string());

        println!("{}", std::mem::size_of::<Color>());
    }
}
