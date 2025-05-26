use eframe::AppCreator;
use eframe::NativeOptions;
use egui::ViewportBuilder;
use srpg_arena::APP_NAME;
pub use srpg_arena::Error;
pub use srpg_arena::Result;
use srpg_arena::app::ArenaApp;

//////////
// Main //
//////////

fn main() -> eframe::Result {
    let viewport = ViewportBuilder::default()
        .with_inner_size([800.0, 600.0])
        .with_min_inner_size([400.0, 300.0]);

    let options = NativeOptions { viewport, ..Default::default() };
    let creator: AppCreator = Box::new(|cc| Ok(Box::new(ArenaApp::new(cc))));
    eframe::run_native(APP_NAME, options, creator)
}
