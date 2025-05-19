pub use srpg_arena::Error;
pub use srpg_arena::Result;
use srpg_arena::app::App;

//////////
// Main //
//////////

fn main() -> crate::Result {
    let mut app = App::new();
    let mut terminal = ratatui::init(); // NB: adds a panic hook for restore()
    let final_result = app.run(&mut terminal);
    ratatui::restore();
    final_result
}
