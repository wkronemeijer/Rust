//! Implements https://midi.org/summary-of-midi-1-0-messages for real-time use
//! The file format is implemented in [`midi_file`].
//!
//! Ideally we use non-power-of-two integer types for this (e.g. u4, u7)
//! But the library for that (`ux`) hasn't been updated
//! AND has no TryFrom<u8> impl for some reason.

use anyhow::Context;
use arrayvec::ArrayVec;
use midir::MidiOutputConnection;

use crate::domain::midi_ux::u4;
use crate::domain::midi_ux::u7;

/////////////////
// MidiChannel //
/////////////////

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
/// A 4-bit number for the channel.
pub struct Channel(pub u4);

impl Channel {
    pub const ONE: Self = Channel(u4::new(0).unwrap());
    pub const TWO: Self = Channel(u4::new(1).unwrap());
    pub const THREE: Self = Channel(u4::new(2).unwrap());
    pub const FOUR: Self = Channel(u4::new(3).unwrap());

    pub const MIN: Self = Channel(u4::MIN);
    pub const MAX: Self = Channel(u4::MAX);

    pub const COUNT: usize = 16;
}

///////////////
// MidiPitch //
///////////////

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Pitch(pub u7);

impl Pitch {
    pub const MIN: Self = Pitch(u7::MIN);
    pub const C4: Self = Pitch(u7::new(60).unwrap());
    pub const A4: Self = Pitch(u7::new(69).unwrap());
    pub const MAX: Self = Pitch(u7::MAX);
}

impl Default for Pitch {
    fn default() -> Self { Self::C4 }
}

//////////////
// MidiNote //
//////////////

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Velocity(pub u7);

impl Velocity {
    pub const MIN: Self = Velocity(u7::MIN);
    pub const DEFAULT: Self = Velocity(u7::new(64).unwrap());
    pub const MAX: Self = Velocity(u7::MAX);
}

impl Default for Velocity {
    fn default() -> Self { Self::DEFAULT }
}

//////////////////
// MIDI Program //
//////////////////

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Program(pub u7);

impl Program {
    pub const DEFAULT: Self = Program(u7::new(0).unwrap());
}

impl Default for Program {
    fn default() -> Self { Self::DEFAULT }
}

/////////////////////
// MIDI Instrument //
/////////////////////

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Instrument {
    GrandPiano = 0,

    Harpischord = 6,

    Marimba = 12,
    Xylophone = 13,

    Ocarina = 79,
    Koto = 107,
}

impl From<Instrument> for Program {
    fn from(instr: Instrument) -> Self {
        Self(u7::new(instr as u8).expect("failed to convert"))
    }
}

////////////////////////////
// (General) MIDI Control //
////////////////////////////

#[derive(Debug, Clone, Copy)]
pub enum ControlEntry {
    Volume(u7),
}

impl ControlEntry {
    pub fn key(self) -> u7 {
        u7::new(match self {
            ControlEntry::Volume(..) => 7,
        })
        .unwrap()
    }

    pub fn value(self) -> u7 {
        match self {
            ControlEntry::Volume(v) => v,
        }
    }
}

/////////////////
// MidiMessage //
/////////////////

#[derive(Debug, Clone, Copy)]
pub enum Message {
    NoteOn(Channel, Pitch, Velocity),
    NoteOff(Channel, Pitch, Velocity),
    ProgramChange(Channel, Program),
    // ...only SysEx doesn't have channel info
    // And SysEx is variable width and very complicated
}

impl Message {
    const NOTE_OFF: u8 = 0b1000_0000;
    const NOTE_ON: u8 = 0b1001_0000;
    const PROGRAM_CHANGE: u8 = 0b1100_0000;
    // const PITCH_BEND_CHANGE: u8 = 0b1110_0000;
}

impl Message {
    pub fn send(self, out: &mut MidiOutputConnection) -> crate::Result {
        use Message::*;
        let mut msg = ArrayVec::<u8, 4>::new();
        match self {
            NoteOn(Channel(ch), Pitch(p), Velocity(v)) => {
                msg.extend([Self::NOTE_ON | *ch, *p, *v])
            },
            NoteOff(Channel(ch), Pitch(p), Velocity(v)) => {
                msg.extend([Self::NOTE_OFF | *ch, *p, *v])
            },
            ProgramChange(Channel(ch), Program(p)) => {
                msg.extend([Self::PROGRAM_CHANGE | *ch, *p])
            },
        };
        out.send(&msg).context("midi error: ")
    }
}

pub trait MessageSink {
    fn send_message(&mut self, msg: Message) -> crate::Result;
}

impl MessageSink for MidiOutputConnection {
    fn send_message(&mut self, msg: Message) -> crate::Result { msg.send(self) }
}
