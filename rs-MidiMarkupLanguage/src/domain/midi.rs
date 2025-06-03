//! Implements https://midi.org/summary-of-midi-1-0-messages for real-time use
//! The file format is implemented in [`midi_file`].

use std::fmt;

use midir::MidiOutputConnection;

// SIDE NOTE: I wish we had u4, u7, etc. in Rust
// It would put the validation outside of the constructors
// Something something parse, don't validate
const U4_MAX: u8 = 0b0000_1111;
const U7_MAX: u8 = 0b0111_1111;

/////////////////
// MidiChannel //
/////////////////

#[derive(Clone, Copy)]
#[repr(transparent)]
/// A 4-bit number for the channel. Unlike MIDI,
pub struct Channel {
    /// Invariant: contains a u4
    offset: u8,
}

impl Channel {
    /// Create a 0-based channel.
    pub fn new(offset: u8) -> Option<Self> {
        if offset <= U4_MAX { Some(Channel { offset }) } else { None }
    }

    pub const ONE: Self = Channel { offset: 0 };
    pub const TWO: Self = Channel { offset: 1 };
    pub const FIFTEEN: Self = Channel { offset: 15 };

    pub const MIN: Self = Self::ONE;
    pub const MAX: Self = Self::FIFTEEN;
}

impl Channel {
    #[inline]
    /// The 0-based index of this channel.
    pub fn offset(&self) -> u8 { self.offset }

    /// The 1-based index of this channel.
    pub fn number(&self) -> u8 { self.offset + 1 }
}

impl fmt::Debug for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Channel").field(&self.offset).finish()
    }
}

///////////////
// MidiPitch //
///////////////

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Pitch {
    value: u8,
}

impl Pitch {
    pub fn new(value: u8) -> Option<Self> {
        if value <= U7_MAX { Some(Pitch { value }) } else { None }
    }

    pub const MIN: Self = Pitch { value: 0 };
    pub const C4: Self = Pitch { value: 60 };
    pub const A4: Self = Pitch { value: 69 };
    pub const MAX: Self = Pitch { value: 127 };
}

impl Pitch {
    #[inline]
    pub fn value(&self) -> u8 { self.value }
}

impl fmt::Debug for Pitch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Pitch").field(&self.value).finish()
    }
}

impl Default for Pitch {
    fn default() -> Self { Self::C4 }
}

//////////////
// MidiNote //
//////////////

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Velocity {
    value: u8,
}

impl Velocity {
    pub const fn new(value: u8) -> Option<Self> {
        if value <= U7_MAX { Some(Velocity { value }) } else { None }
    }

    pub const MIN: Self = Velocity { value: 0 };
    pub const DEFAULT: Self = Velocity { value: 64 };
    pub const MAX: Self = Velocity { value: 127 };
}

impl Velocity {
    #[inline]
    pub fn value(&self) -> u8 { self.value }
}

impl fmt::Debug for Velocity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Velocity").field(&self.value).finish()
    }
}

impl Default for Velocity {
    fn default() -> Self { Self::DEFAULT }
}

/////////////////
// MidiMessage //
/////////////////

#[derive(Debug, Clone, Copy)]
pub enum Message {
    NoteOn(Channel, Pitch, Velocity),
    NoteOff(Channel, Pitch, Velocity),
}

impl Message {
    const NOTE_OFF: u8 = 0b1000_0000;
    const NOTE_ON: u8 = 0b1001_0000;
}

impl Message {
    pub fn send_to(self, out: &mut MidiOutputConnection) -> crate::Result {
        match self {
            Message::NoteOn(channel, pitch, velocity) => out.send(&[
                Self::NOTE_ON | channel.offset(),
                pitch.value(),
                velocity.value(),
            ]),
            Message::NoteOff(channel, pitch, velocity) => out.send(&[
                Self::NOTE_OFF | channel.offset(),
                pitch.value(),
                velocity.value(),
            ]),
        }?;
        Ok(())
    }
}

pub trait MessageSink {
    fn send_message(&mut self, msg: Message) -> crate::Result;
}

impl MessageSink for MidiOutputConnection {
    fn send_message(&mut self, msg: Message) -> crate::Result {
        msg.send_to(self)
    }
}
