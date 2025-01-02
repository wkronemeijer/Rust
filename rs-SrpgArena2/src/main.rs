use std::panic::catch_unwind;
use std::time::Duration;
use std::time::Instant;

use ratatui::DefaultTerminal;
use ratatui::Frame;
use ratatui::crossterm::event::Event;
use ratatui::crossterm::event::KeyCode;
use ratatui::crossterm::event::KeyEvent;
use ratatui::crossterm::event::poll;
use ratatui::crossterm::event::read;
use ratatui::widgets::Block;
use ratatui::widgets::BorderType;
use ratatui::widgets::Borders;
use ratatui::widgets::Paragraph;
pub use srpg_arena::Result;
use srpg_arena::events::ObserverInstance;
use srpg_arena::game::Arena;
use srpg_arena::game::Unit;
use srpg_arena::stats::Stats;

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

fn run() {
    let mut arena = init_arena();

    let mut observer = ObserverInstance::new();

    let before = Instant::now();
    let result = arena.fight_to_the_death(&mut observer);
    let after = Instant::now();

    match result.victor {
        Some(unit) => println!(
            "{} is victorious!",
            arena.combatants.get(unit).unwrap().name()
        ),
        None => println!("Everyone died..."),
    }

    let millis = (after - before).as_millis();
    println!("(completed in {}ms)", millis)
}

pub enum ExitReason {
    Success,
    Failure(String),
}

struct App {
    should_exit: Option<ExitReason>, // TODO: Put anyhow's type in here
    arena: Option<Arena>,
}

impl App {
    pub fn new() -> Self { App { should_exit: None, arena: None } }

    fn advance(&mut self) {
        match self.arena {
            None => self.arena = Some(init_arena()),
            Some(ref mut arena) => {
                todo!();
            }
        }
    }

    fn exit(&mut self, reason: ExitReason) { self.should_exit = Some(reason) }

    fn draw(&self, frame: &mut Frame) {
        let widget = Paragraph::new("Hello, world!").block(
            Block::new().borders(Borders::ALL).border_type(BorderType::Double),
        );
        frame.render_widget(widget, frame.area());
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Enter => self.advance(),
                KeyCode::F(8) => self.exit(ExitReason::Success),
                _ => {}
            },
            _ => {}
        }
    }

    fn handle_events(&mut self) {
        const ZERO: Duration = Duration::from_secs(0);
        loop {
            let Ok(true) = poll(ZERO) else { break };
            let Ok(event) = read() else { continue };
            // TODO: Should we exit based on IO errors?
            self.handle_event(event);
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> ExitReason {
        loop {
            if let Some(reason) = self.should_exit.take() {
                return reason;
            }
            if let Err(e) = terminal.draw(|f| self.draw(f)) {
                return ExitReason::Failure(e.to_string());
            }
            self.handle_events();
        }
    }
}

//////////
// Main //
//////////

fn main() -> crate::Result {
    let mut terminal = ratatui::init();
    let result = catch_unwind(move || {
        let mut app = App::new();
        app.run(&mut terminal)
    });
    ratatui::restore();
    match result {
        Ok(ExitReason::Failure(message)) => {
            Err(format!("exit caused by '{}'", message).into())
        }
        Err(err) => {
            Err(format!("panic caused by '{:?}'", err.type_id()).into())
        }
        _ => Ok(()),
    }
}
