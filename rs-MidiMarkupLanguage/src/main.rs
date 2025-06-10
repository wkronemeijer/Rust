use std::env::args;
use std::thread::sleep;
use std::time::Duration;

use anyhow::Context;
use anyhow::bail;
pub use midimlc::Error;
pub use midimlc::Result;
use midimlc::midi::common::Channel;
use midimlc::midi::common::Message;
use midimlc::midi::common::MessageSink;
use midimlc::midi::common::Pitch;
use midimlc::midi::common::Program;
use midimlc::midi::common::Velocity;
use midimlc::midi::int::u7;
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

fn fun_name(out: &mut MidiOutputConnection, instrument: u7) -> crate::Result {
    let ch = Channel::ONE;
    let vel = Velocity::MAX;
    const MILLIS_PER_TICK: u64 = 150;

    out.send_message(Message::ProgramChange(ch, Program(instrument)))?;

    let mut play_note = |p: Pitch, ticks: u64| -> crate::Result {
        out.send_message(Message::NoteOn(ch, p, vel))?;
        sleep(Duration::from_millis(MILLIS_PER_TICK * ticks));
        out.send_message(Message::NoteOff(ch, p, vel))?;
        Ok(())
    };

    play_note(Pitch(u7::new(66).unwrap()), 4)?;
    play_note(Pitch(u7::new(65).unwrap()), 3)?;
    play_note(Pitch(u7::new(63).unwrap()), 1)?;
    play_note(Pitch(u7::new(61).unwrap()), 6)?;
    play_note(Pitch(u7::new(59).unwrap()), 2)?;
    play_note(Pitch(u7::new(58).unwrap()), 4)?;
    play_note(Pitch(u7::new(56).unwrap()), 4)?;
    play_note(Pitch(u7::new(54).unwrap()), 4)?;

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
    fun_name(&mut out, instrument)?;
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
