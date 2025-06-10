//! Implements data types for writing a MIDI file.
//!
//! Based on https://ccrma.stanford.edu/~craig/14q/midifile/MidiFileFormat.html

////////////////////////
// Time-related decls //
////////////////////////

//////////////
// MidiTime //
//////////////

// Time problems:

use std::cmp::Ordering;
use std::ops::Sub;

use midir::MidiOutputConnection;

use crate::midi::common::Message;

#[derive(Debug, Clone, Copy)]
pub struct MidiTime {
    // Requires PPQ and BPM to turn into ms
    ticks_since_origin: u32,
}

impl MidiTime {
    pub fn ticks_since_origin(&self) -> u32 { self.ticks_since_origin }
}

impl PartialEq for MidiTime {
    fn eq(&self, other: &Self) -> bool { self.cmp(other) == Ordering::Equal }
}

impl Eq for MidiTime {}

impl PartialOrd for MidiTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for MidiTime {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ticks_since_origin.cmp(&other.ticks_since_origin)
    }
}

impl Sub for MidiTime {
    type Output = MidiDuration;

    fn sub(self, rhs: Self) -> Self::Output {
        MidiDuration::new(
            self.ticks_since_origin
                .saturating_sub(rhs.ticks_since_origin)
                .try_into()
                .unwrap_or(i32::MAX),
        )
    }
}

//////////////////
// MidiDuration //
//////////////////

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct MidiDuration {
    ticks: i32,
}

impl MidiDuration {
    pub const fn new(difference: i32) -> MidiDuration {
        MidiDuration { ticks: difference }
    }

    pub fn ticks(&self) -> i32 { self.ticks }
}

pub struct TimedMessage {
    pub message: Message,
    pub time: MidiTime,
}

pub fn sequence_messages(
    _out: &mut MidiOutputConnection,
    mut messages: Vec<TimedMessage>,
) -> crate::Result {
    messages.sort_by_key(|m| m.time);
    todo!()
}
