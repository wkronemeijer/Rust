//! Items to display a progress bar / spinner.

use std::io::BufWriter;
use std::io::Write;
use std::io::stderr;
use std::thread;
use std::thread::Scope;
use std::thread::ScopedJoinHandle;
use std::time::Duration;

use crate::core::ansi::CLEAR_LINE;
use crate::core::ansi::HIDE_CURSOR;
use crate::core::ansi::SHOW_CURSOR;
use crate::core::sync::CancellationToken;

// From https://github.com/sindresorhus/cli-spinners/blob/main/spinners.json
pub const BRAILLE_CIRCLE: &[&str] =
    &["⢎ ", "⠎⠁", "⠊⠑", "⠈⠱", " ⡱", "⢀⡰", "⢄⡠", "⢆⡀"];

pub fn spawn_spinner<'s, 'e>(
    scope: &'s Scope<'s, 'e>,
    frames: &'e [&'e str],
    period: Duration,
    token: CancellationToken,
) -> ScopedJoinHandle<'s, ()> {
    const CAPACITY: usize = 1 << 8;

    let frame_count = frames.len() as u32;
    let time_per_step = period / frame_count;

    scope.spawn(move || {
        let stderr = stderr().lock();
        let mut out = BufWriter::with_capacity(CAPACITY, stderr);
        // ↑ buffer stderr to prevent flickering

        let mut frame_loop = frames.into_iter().cycle();

        write!(out, "{}", HIDE_CURSOR).unwrap();
        while let Some(&char) = frame_loop.next() {
            write!(out, "{}\r", CLEAR_LINE).unwrap();
            if token.cancelled() {
                break;
            }
            write!(out, "{}", char).unwrap();
            out.flush().unwrap();
            thread::sleep(time_per_step);
        }
        write!(out, "{}", SHOW_CURSOR).unwrap();
        out.flush().unwrap();
    })
}
