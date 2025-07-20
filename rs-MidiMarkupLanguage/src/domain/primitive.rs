use std::fmt;

use int_enum::IntEnum;

use crate::core::ratio::Ratio;

/////////////////
// Pitch Class //
/////////////////

#[derive(Debug, Clone, Copy, IntEnum)]
#[repr(u8)]
pub enum PitchClass {
    C,
    Cs,
    D,
    Ds,
    E,
    // E# == F
    F,
    Fs,
    G,
    Gs,
    A,
    As,
    B,
    // B# == C
}

impl PitchClass {
    // TODO: How to statically assert this?
    pub const COUNT: usize = 12;

    pub fn from_offset(value: u8) -> Option<Self> { value.try_into().ok() }
    pub fn offset(self) -> u8 { self.into() }
}

impl fmt::Display for PitchClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(match self {
            PitchClass::C => "C",
            PitchClass::Cs => "C#",
            PitchClass::D => "D",
            PitchClass::Ds => "D#",
            PitchClass::E => "E",
            PitchClass::F => "F",
            PitchClass::Fs => "F#",
            PitchClass::G => "G",
            PitchClass::Gs => "G#",
            PitchClass::A => "A",
            PitchClass::As => "A#",
            PitchClass::B => "B",
        })
    }
}

/////////////////////////
// Semitone difference //
/////////////////////////

/// Counts the difference in semitones between two pitches.
#[derive(Clone, Copy)]
pub struct Interval {
    semis: u8,
}

impl Interval {
    // https://en.wikipedia.org/wiki/Interval_(music)#Main_intervals
    pub const UNISON: Self = Interval { semis: 0 };
    pub const HALF: Self = Interval { semis: 1 };
    pub const WHOLE: Self = Interval { semis: 2 };
    pub const TRITONE: Self = Interval { semis: 6 };
    pub const OCTAVE: Self = Interval { semis: 12 };
}

///////////
// Pitch //
///////////

// Technically octave no keep going (C-1000 is a thing)
// MIDI goes from C0 to A9 (roughly)
pub type Octave = i8;
// An alias because octaves really have no special notation or other use

#[derive(Clone, Copy)]
pub struct Pitch {
    // Pitch stored as semitones above C0
    // Yes, C0 (== 16Hz) is the floor
    value: u8,
    // NB: does not model cents
}

impl Pitch {
    pub const fn new(class: PitchClass, octave: Octave) -> Self {
        Pitch { value: (octave * 12) as u8 + class as u8 }
    }

    pub fn decompose(self) -> (PitchClass, Octave) { todo!() }

    pub fn frequency(self) -> f64 {
        // A4 == 440Hz (which is but a standard) (can't have everything vary)
        todo!()
    }
}

impl Pitch {
    pub const A4: Self = Pitch::new(PitchClass::A, 4);
    pub const C4: Self = Pitch::new(PitchClass::C, 4);
}

//////////////
// Duration //
//////////////

#[derive(Clone, Copy)]
pub struct NoteValue {
    value: Ratio<i32>,
}

impl NoteValue {
    pub const WHOLE: Self = NoteValue { value: Ratio::ONE };
    pub const HALF: Self = NoteValue { value: Ratio::new(1, 2) };
    pub const QUARTER: Self = NoteValue { value: Ratio::new(1, 4) };
    pub const EIGHTH: Self = NoteValue { value: Ratio::new(1, 4) };

    /// Extends duration by a half
    pub fn dot(self) -> Self {
        static SESQUI: Ratio<i32> = Ratio::new(3, 2);
        NoteValue { value: self.value * SESQUI }
    }
}

/////////////////
// Arrangement //
/////////////////

// impl FromStr for Pitch?

/// An arrangement with abstract positions, like anchors and until-end-of-measure
pub enum AbstractArrangement {
    Rest(),
    Note(),
    Harmony(),
    Melody(),
}

// Is Vec a bad idea?
// We need some kind of traversable list
// Maybe think about this later

/// An arrangement where all lengths have been resolved.
pub enum ConcreteArrangement {
    Rest(),
    Note(),
    Harmony(),
    Melody(),
}
