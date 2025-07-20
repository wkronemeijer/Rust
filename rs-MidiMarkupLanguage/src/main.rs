use std::env::args;
use std::thread::sleep;
use std::time::Duration;

use anyhow::Context;
use anyhow::bail;
pub use midimlc::Error;
pub use midimlc::Result;
use midimlc::core::int::u7;
use midimlc::midi::common::Channel;
use midimlc::midi::common::ControlEntry;
use midimlc::midi::common::Message;
use midimlc::midi::common::MessageSink;
use midimlc::midi::common::Pitch;
use midimlc::midi::common::Program;
use midimlc::midi::common::Velocity;
use midir::MidiOutput;
use midir::MidiOutputConnection;
use midir::MidiOutputPort;

fn discover_output_port(
    ports: &[MidiOutputPort],
) -> crate::Result<&MidiOutputPort> {
    Ok(match &*ports {
        [] => bail!("no output port found"),
        [port] => port,
        _ => bail!("more than 1 output port found"),
    })
}

fn send_track(out: &mut MidiOutputConnection, instrument: u7) -> crate::Result {
    let ch = Channel::ONE;
    let vel = Velocity::MAX;
    const MILLIS_PER_TICK: u64 = 150;

    out.send_message(Message::ProgramChange(ch, Program(instrument)))?;
    out.send_message(Message::ControlChange(ch, ControlEntry::Pedal(true)))?;

    let mut play_note = |p: Pitch, ticks: u64| -> crate::Result {
        out.send_message(Message::NoteOn(ch, p, vel))?;
        sleep(Duration::from_millis(MILLIS_PER_TICK * ticks));
        out.send_message(Message::NoteOff(ch, p, vel))?;
        Ok(())
    };

    // Walk down F# Ionian
    play_note(Pitch(u7::new(66).unwrap()), 4)?; // F#
    play_note(Pitch(u7::new(65).unwrap()), 3)?; // F
    play_note(Pitch(u7::new(63).unwrap()), 1)?; // D#
    play_note(Pitch(u7::new(61).unwrap()), 6)?; // C#
    play_note(Pitch(u7::new(59).unwrap()), 2)?; // B
    play_note(Pitch(u7::new(58).unwrap()), 4)?; // A#
    play_note(Pitch(u7::new(56).unwrap()), 4)?; // G#
    play_note(Pitch(u7::new(54).unwrap()), 4)?; // F#

    Ok(())
}

fn run() -> crate::Result {
    let mut args = args().skip(1);
    let instrument = match args.next() {
        Some(no) => u7::new(u8::from_str_radix(&no, 10)? - 1)
            .context("invalid instrument")?,
        None => u7::default(),
    };

    let out = MidiOutput::new("MidiML Runner")?;
    let ports = out.ports();
    let out_port = discover_output_port(&ports)?;
    let out_name = out.port_name(&out_port)?; // store before consuming `out`
    let mut out = out.connect(&out_port, "MidiML Runner Connection")?;

    println!("using '{}'", out_name);

    // Fade in
    sleep(Duration::from_millis(150));
    // Play thing
    send_track(&mut out, instrument)?;
    // Fade out
    sleep(Duration::from_millis(300));

    Ok(())
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("\x1b[31merror: {}\x1b[39m", err),
    }
}
