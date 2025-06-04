use eframe::App;
use eframe::CreationContext;
use egui::CentralPanel;
use egui::Color32;
use egui::FontDefinitions;
use egui::FontFamily;
use egui::Key;
use egui::ProgressBar;
use egui::RichText;
use egui::Theme;
use egui::ViewportCommand;

use crate::assets::fonts::FONTIN_REGULAR;
use crate::assets::fonts::FONTIN_SMALL_CAPS;
use crate::game::Arena;
use crate::game::Unit;
use crate::stats::Stats;

fn init_arena() -> Arena {
    let mut arena = Arena::new();

    arena.add({
        let mut stats = Stats::default();

        stats.maximum_life = 20;
        stats.phys_damage = 9;
        stats.phys_defense = 4;
        stats.attack_speed = 3;
        stats.hit = 90;
        stats.avoid = 35;

        stats.crit = 15;

        Unit::new("Aerith", stats)
    });

    arena.add({
        let mut stats = Stats::default();

        stats.maximum_life = 30;
        stats.phys_damage = 11;
        stats.phys_defense = 5;
        stats.attack_speed = 3;
        stats.hit = 85;
        stats.avoid = 15;

        stats.crit = 5;

        Unit::new("Bob", stats)
    });

    arena
}

pub struct ArenaApp {
    arena: Option<Arena>,
}

impl ArenaApp {
    pub fn new(cc: &CreationContext) -> Self {
        cc.egui_ctx.set_zoom_factor(16.0 / 12.0); // bad eyes ok
        cc.egui_ctx.set_theme(Theme::Dark);
        cc.egui_ctx.set_fonts({
            // TODO: Could we copy the current `cc.egui_ctx.fonts`?
            let mut fonts = FontDefinitions::default();

            FONTIN_REGULAR.register(&mut fonts);
            FONTIN_REGULAR.register_as(&mut fonts, FontFamily::Proportional);
            FONTIN_SMALL_CAPS.register(&mut fonts);
            fonts
        });

        ArenaApp { arena: None }
    }
}

impl App for ArenaApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            if ui.input(|i| i.key_pressed(Key::F8)) {
                ctx.send_viewport_cmd(ViewportCommand::Close);
            }

            ui.heading("Header");

            ui.add(ProgressBar::new(0.5));

            if self.arena.is_none() {
                if ui.button("Create arena").clicked() {
                    self.arena.get_or_insert_with(init_arena);
                }
            } else {
                if ui.button("Destroy arena").clicked() {
                    self.arena = None;
                }
            }

            if let Some(arena) = &mut self.arena {
                if ui.button("Reset").clicked() {
                    arena.reset();
                }

                for (_, unit) in arena.combatants().entries() {
                    ui.group(|ui| {
                        ui.label(
                            RichText::new(unit.name())
                                .font(FONTIN_SMALL_CAPS.sized(12.0)),
                        );

                        let min_hp = unit.resources().life;
                        let max_hp = 40i16;

                        let ratio = min_hp as f32 / max_hp as f32;
                        let text = format!("{}/{}", min_hp, max_hp);
                        let bar = ProgressBar::new(ratio)
                            .text(text)
                            .corner_radius(0)
                            .fill(Color32::from_rgb(255, 0, 0));
                        ui.add(bar);
                    });
                }

                ui.code(format!("{:#?}", arena));
            }
        });
    }
}
