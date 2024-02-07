mod bounded;
mod error;
mod game;

use cursive::views::TextView;
use cursive::{Cursive, CursiveExt};

fn main() {
    let mut siv = Cursive::new();
    siv.add_layer(TextView::new("Hello World!\nPress q to quit."));
    siv.add_global_callback('q', |s| s.quit());
    siv.run();
}
