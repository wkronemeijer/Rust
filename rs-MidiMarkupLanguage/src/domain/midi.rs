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
        let mut msg = ArrayVec::<u8, 4>::new();

        // Ideally, we put the `send()` outside
        // Lifetimes don't allow it
        match self {
            Message::NoteOn(channel, pitch, velocity) => {
                msg.extend([Self::NOTE_ON | *channel.0, *pitch.0, *velocity.0])
            },
            Message::NoteOff(channel, pitch, velocity) => {
                msg.extend([Self::NOTE_OFF | *channel.0, *pitch.0, *velocity.0])
            },
        };

        out.send(&msg).context("midi error: ")
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
