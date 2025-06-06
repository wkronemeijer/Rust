use std::thread::sleep;
use std::time::Duration;

use anyhow::bail;
pub use midimlc::Error;
pub use midimlc::Result;
use midimlc::domain::midi::Channel;
use midimlc::domain::midi::Instrument;
use midimlc::domain::midi::Message;
use midimlc::domain::midi::MessageSink;
use midimlc::domain::midi::Pitch;
use midimlc::domain::midi::Velocity;
use midimlc::domain::midi_ux::u7;
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

fn fun_name(out: &mut MidiOutputConnection) -> crate::Result {
    let ch = Channel::ONE;
    let vel = Velocity::MAX;
    const MILLIS_PER_TICK: u64 = 150;

    Message::ProgramChange(ch, Instrument::Marimba.into()).send(out)?;

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
    let out = MidiOutput::new("MidiML Runner")?;
    let ports = out.ports();
    let out_port = discover_output_port(&ports)?;
    let out_name = out.port_name(&out_port)?; // store before consuming `out`
    let mut out = out.connect(&out_port, "MidiML Runner Connection")?;

    println!("using '{}'", out_name);

    // Fade in
    sleep(Duration::from_millis(150));
    // Play thing
    fun_name(&mut out)?;
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
