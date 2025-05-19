//! Items to display a progress bar / spinner.

use std::io::BufWriter;
use std::io::IsTerminal;
use std::io::Write;

use crate::core::ansi::CLEAR_LINE;

#[derive(Debug)]
/// Wraps a writer for use as a ANSI status line.
pub struct StatusLine<W: Write> {
    out: Option<BufWriter<W>>,
}

impl<W: Write + IsTerminal> StatusLine<W> {
    const CAPACITY: usize = 1 << 8;

    /// Creates a new instance using a writer.
    pub fn new(writer: W) -> Self {
        let out = if writer.is_terminal() {
            Some(BufWriter::with_capacity(Self::CAPACITY, writer))
        } else {
            None
        };
        StatusLine { out }
    }

    /// Writes a status line.
    /// Everything after the first line break is ignored.
    pub fn writeln(&mut self, line: &str) {
        let Some(out) = &mut self.out else { return };
        let Some(first_line) = line.lines().next() else { return };
        write!(out, "{}\r{}", CLEAR_LINE, first_line).unwrap();
        // FIXME: #lines > #cols still causes issues.
        out.flush().unwrap();
    }

    /// Closes the status line.
    pub fn close(mut self) {
        let Some(out) = &mut self.out else { return };
        write!(out, "{}\r", CLEAR_LINE).unwrap();
    }
}
