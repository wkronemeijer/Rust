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

/// (C)ontrol (S)equence (I)nitiator
pub const CSI: &str = "\x1B[";
/// (O)perating (S)system (C)ommand
pub const OSC: &str = "\x1B]";
/// (S)tring (T)erminator
pub const ST: &str = "\x1B\\";

/// ANSI sequence to clear the entire line.
pub const CLEAR_LINE: &str = "\x1B[2K";
/// ANSI sequence to hide the cursor.
pub const HIDE_CURSOR: &str = "\x1B[?25l";
/// ANSI sequence to show the cursor.
pub const SHOW_CURSOR: &str = "\x1B[?25h";

/////////////////////
// Simple wrappers //
/////////////////////

macro_rules! sgr_wrapper {
    ($ty:ident, $on:literal, $off:literal) => {
        #[derive(Debug)]
        #[repr(transparent)]
        #[doc = concat!(
                                                    "ANSI sequence to apply ",
                                                    stringify!($ty),
                                                    " to a span of text."
                                                )]
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
/// Where to apply the color.
pub enum ColorTarget {
    /// Apply the color to the foreground/letter/fill.
    Foreground,
    /// Apply the color to the background.
    Background,
}

impl ColorTarget {
    /// The base value to use for a given target.
    pub fn base(self) -> u8 {
        match self {
            Self::Foreground => 30,
            Self::Background => 40,
        }
    }

    /// Undo the color
    pub fn reset(self) -> u8 { self.base() + 9 }
}

/// All things usable as ANSI colors.
pub trait Color {
    /// Generates the SGR sequence to set the given color.
    fn args(&self, target: ColorTarget) -> impl Iterator<Item = u8>;
}

/////////////////
// 3-bit color //
/////////////////

#[derive(Debug, Clone, Copy)]
/// Basic, 3-bit ANSI colors.
pub enum AnsiColor {
    /// Black.
    Black = 0,
    /// Red.
    Red = 1,
    /// Green.
    Green = 2,
    /// Yellow.
    Yellow = 3,
    /// Blue.
    Blue = 4,
    /// Magenta.
    Magenta = 5,
    /// Cyan.
    Cyan = 6,
    /// White.
    White = 7,
}

impl AnsiColor {
    /// The offset from the black for this given color.
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
/// ANSI 3-bit colors, but brighter.
pub struct BrightAnsiColor(pub AnsiColor);

#[expect(non_upper_case_globals)]
impl BrightAnsiColor {
    /// Bright Black. Typically displays as dark gray.
    pub const Black: BrightAnsiColor = Self(AnsiColor::Black);
    /// Bright Red.
    pub const Red: BrightAnsiColor = Self(AnsiColor::Red);
    /// Bright Green.
    pub const Green: BrightAnsiColor = Self(AnsiColor::Green);
    /// Bright Yellow.
    pub const Yellow: BrightAnsiColor = Self(AnsiColor::Yellow);
    /// Bright Blue.
    pub const Blue: BrightAnsiColor = Self(AnsiColor::Blue);
    /// Bright Magenta.
    pub const Magenta: BrightAnsiColor = Self(AnsiColor::Magenta);
    /// Bright Cyan.
    pub const Cyan: BrightAnsiColor = Self(AnsiColor::Cyan);
    /// Bright White. Typically displays as light gray.
    pub const White: BrightAnsiColor = Self(AnsiColor::White);
}

impl AnsiColor {
    /// Upgrades an 3-bit ANSI color to its brighter variant.
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
/// Full, 24-bit RGB ANSI color.
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
/// A colored span of text.
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
/// A hypertext anchor, which contains a URI to somewhere.
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
/// When outputted, sets the title of the window.
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
/// When outputted, sets the progress of the window.
/// On Windows, this displays as a bar over the taskbar icon.
pub enum SetProgress {
    #[default]
    /// Process is complete.
    Done,
    /// Process is continueing, and currently at the given percentage.
    Continue(u8),
    /// Process is paused, and currently at the given percentage.
    Paused(u8),
    /// Process has failed at the given percentage.
    Error(u8),
    /// Process progress is unknown.
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
