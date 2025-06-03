use std::thread::sleep;
use std::time::Duration;

use anyhow::bail;
pub use midimlc::Error;
pub use midimlc::Result;
use midimlc::domain::midi::Channel;
use midimlc::domain::midi::Message;
use midimlc::domain::midi::MessageSink;
use midimlc::domain::midi::Pitch;
use midimlc::domain::midi::Velocity;
use midir::MidiOutput;
use midir::MidiOutputPort;

fn discover_output_port(
    midi_out: &MidiOutput,
) -> crate::Result<MidiOutputPort> {
    let mut out_ports = midi_out.ports();
    Ok(match out_ports.len() {
        0 => bail!("no output port found"),
        1 => out_ports.swap_remove(0),
        _ => bail!("more than 1 output port found"),
    })
}

fn run() -> crate::Result {
    let out = MidiOutput::new("MidiML Runner")?;
    let out_port = discover_output_port(&out)?;
    let out_name = out.port_name(&out_port)?; // store before consuming `out`
    let mut out = out.connect(&out_port, "MidiML Runner Connection")?;
    println!("outputting to {}", out_name);

    let ch = Channel::ONE;
    let vel = Velocity::new(32).unwrap();
    // Define a new scope in which the closure `play_note` borrows conn_out, so it can be called easily
    let mut play_note = |pitch: Pitch, duration: u64| -> crate::Result {
        let msg = Message::NoteOn(ch, pitch, vel);
        eprintln!("{:?}", msg);
        out.send_message(msg)?;
        sleep(Duration::from_millis(duration * 150));
        out.send_message(Message::NoteOff(ch, pitch, vel))?;
        Ok(())
    };

    // Fade in
    sleep(Duration::from_millis(150));

    // Play a song
    play_note(Pitch::new(66).unwrap(), 4)?;
    play_note(Pitch::new(65).unwrap(), 3)?;
    play_note(Pitch::new(63).unwrap(), 1)?;
    play_note(Pitch::new(61).unwrap(), 6)?;
    play_note(Pitch::new(59).unwrap(), 2)?;
    play_note(Pitch::new(58).unwrap(), 4)?;
    play_note(Pitch::new(56).unwrap(), 4)?;
    play_note(Pitch::new(54).unwrap(), 4)?;

    // Fade out
    sleep(Duration::from_millis(300));

    Ok(())
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("\x1b[31merror: {}\x1b[0m", err),
    }
}
