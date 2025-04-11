//! Items to wrap arguments with ANSI terminal sequences,
//! similar to [`::std::path::Display`].
//!
//! Based on https://en.wikipedia.org/wiki/ANSI_escape_code.

use std::fmt;
use std::fmt::Display;
use std::iter::once;

use url::Url;

///////////////
// Constants //
///////////////

pub const CSI: &str = "\x1B[";
pub const OSC: &str = "\x1B]";
pub const ST: &str = "\x1B\\";

pub const CLEAR_LINE: &str = "\x1B[2K";
pub const HIDE_CURSOR: &str = "\x1B[?25l";
pub const SHOW_CURSOR: &str = "\x1B[?25h";

/////////////////////
// Simple wrappers //
/////////////////////

macro_rules! sgr_wrapper {
    ($ty:ident, $on:literal, $off:literal) => {
        #[derive(Debug)]
        #[repr(transparent)]
        pub struct $ty<T>(pub T);

        impl<T: Display> Display for $ty<T> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                let $ty(inner) = self;
                // concat! doesn't like it if I put CSI in here
                write!(f, concat!("\x1B[", $on, "m{}\x1B[", $off, "m"), inner)
            }
        }
    };
}

sgr_wrapper!(Bold, 1, 22); // off != 21 (that is double underline)
sgr_wrapper!(Faint, 2, 22);
sgr_wrapper!(Italic, 3, 23);
sgr_wrapper!(Underlined, 4, 24);
sgr_wrapper!(Inverted, 7, 27);
sgr_wrapper!(Hidden, 8, 28);
sgr_wrapper!(Deleted, 9, 29);

////////////
// Colors //
////////////
// CSI 32 m ==> color: green;
// CSI 49 m ==> background-color: unset;
// CSI 96 m ==> color: bright-cyan;
// CSI 38;2;255;0;0 m ==> color: #FF0000;

#[derive(Debug, Clone, Copy)]
pub enum ColorTarget {
    Foreground,
    Background,
}

impl ColorTarget {
    pub fn base(self) -> u8 {
        match self {
            Self::Foreground => 30,
            Self::Background => 40,
        }
    }

    pub fn reset(self) -> u8 { self.base() + 9 }
}

pub trait Color {
    fn args(&self, target: ColorTarget) -> impl Iterator<Item = u8>;
}

/////////////////
// 3-bit color //
/////////////////

#[derive(Debug, Clone, Copy)]
pub enum AnsiColor {
    Black = 0,
    Red = 1,
    Green = 2,
    Yellow = 3,
    Blue = 4,
    Magenta = 5,
    Cyan = 6,
    White = 7,
}

impl AnsiColor {
    pub fn offset(self) -> u8 { self as u8 }
}

impl Color for AnsiColor {
    fn args(&self, target: ColorTarget) -> impl Iterator<Item = u8> {
        once(target.base() + self.offset())
    }
}

/////////////////
// 4-bit color //
/////////////////

#[derive(Debug, Clone, Copy)]
pub struct BrightAnsiColor(pub AnsiColor);

#[expect(non_upper_case_globals)]
impl BrightAnsiColor {
    pub const Black: BrightAnsiColor = Self(AnsiColor::Black);
    pub const Red: BrightAnsiColor = Self(AnsiColor::Red);
    pub const Green: BrightAnsiColor = Self(AnsiColor::Green);
    pub const Yellow: BrightAnsiColor = Self(AnsiColor::Yellow);
    pub const Blue: BrightAnsiColor = Self(AnsiColor::Blue);
    pub const Magenta: BrightAnsiColor = Self(AnsiColor::Magenta);
    pub const Cyan: BrightAnsiColor = Self(AnsiColor::Cyan);
    pub const White: BrightAnsiColor = Self(AnsiColor::White);
}

impl AnsiColor {
    pub fn bright(self) -> BrightAnsiColor { BrightAnsiColor(self) }
}

impl Color for BrightAnsiColor {
    fn args(&self, target: ColorTarget) -> impl Iterator<Item = u8> {
        let BrightAnsiColor(c) = *self;
        once(target.base() + c.offset() + 60)
    }
}

/////////////////
// 8-bit color //
/////////////////

// Not implemented (yet)
// Why pick 8-bit over 24-bit?

//////////////////
// 24-bit color //
//////////////////

#[derive(Debug, Clone, Copy)]
pub struct RgbColor(pub u8, pub u8, pub u8);

impl Color for RgbColor {
    fn args(&self, target: ColorTarget) -> impl Iterator<Item = u8> {
        let RgbColor(r, g, b) = *self;
        [target.base() + 8, 2, r, g, b].into_iter()
    }
}

///////////////////
// Color styling //
///////////////////

#[derive(Debug)]
pub struct Colored<'a, C, T: ?Sized>(pub ColorTarget, pub C, pub &'a T);

impl<'a, C: Color, T: Display + ?Sized> Display for Colored<'a, C, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Colored(target, color, content) = self;

        // Set (background) color
        CSI.fmt(f)?;
        let mut iter = color.args(*target);
        if let Some(first) = iter.next() {
            first.fmt(f)?;
            for rest in iter {
                ";".fmt(f)?;
                rest.fmt(f)?;
            }
        }
        "m".fmt(f)?;

        // Write content
        content.fmt(f)?;

        // Reset (background) color
        CSI.fmt(f)?;
        target.reset().fmt(f)?;
        "m".fmt(f)
    }
}

////////////
// Anchor //
////////////

#[derive(Debug)]
pub struct Anchor<'a, T: Display>(pub &'a Url, pub T);

impl<'a, T: Display> Display for Anchor<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Anchor(href, content) = self;
        write!(f, "{OSC}8;;{href}{ST}{content}{OSC}8;;{ST}")
    }
}

//////////////////////
// Set window title //
//////////////////////

#[derive(Debug)]
pub struct SetWindowTitle<T>(pub T);

impl<T: Display> Display for SetWindowTitle<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let SetWindowTitle(inner) = self;
        write!(f, "{OSC}0;{inner}{ST}")
    }
}

//////////////////
// Set progress //
//////////////////

#[derive(Debug, Default, Clone, Copy)]
pub enum SetProgress {
    #[default]
    Done,
    Continue(u8),
    Paused(u8),
    Error(u8),
    Indeterminate,
}

impl Display for SetProgress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (code, mut pct) = match *self {
            SetProgress::Done => (0, 0),
            SetProgress::Continue(pct) => (1, pct),
            SetProgress::Error(pct) => (2, pct),
            SetProgress::Indeterminate => (3, 0),
            SetProgress::Paused(pct) => (4, pct),
        };
        pct = pct.clamp(0, 100);
        write!(f, "{OSC}9;4;{code};{pct}{ST}")
    }
}
