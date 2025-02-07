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

pub const BRAILLE_6: &str = concat!(
    '\u{2837}', '\u{282F}', '\u{281F}', '\u{283B}', '\u{283D}', '\u{283E}',
);
pub const BRAILLE_8: &str = concat!(
    '\u{28F7}', '\u{28EF}', '\u{28DF}', '\u{287F}', '\u{28BF}', '\u{28FB}',
    '\u{28FD}', '\u{28FE}',
);

pub fn spawn_spinner<'s, 'e>(
    scope: &'s Scope<'s, 'e>,
    symbols: &'e str,
    period: Duration,
    token: CancellationToken,
) -> ScopedJoinHandle<'s, ()> {
    const CAPACITY: usize = 1 << 8;

    scope.spawn(move || {
        let stderr = stderr().lock();
        // Buffer stderr to prevent flickering
        let mut out = BufWriter::with_capacity(CAPACITY, stderr);

        let mut chars = symbols.chars().cycle();
        let symbol_count = symbols.chars().count() as u32;
        let time_per_step = period / symbol_count;

        write!(out, "{}", HIDE_CURSOR).unwrap();
        while let Some(char) = chars.next() {
            write!(out, "{}\r", CLEAR_LINE).unwrap(); // Note the \r
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
