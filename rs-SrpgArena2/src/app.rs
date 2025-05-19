use std::num::Wrapping;
use std::time::Duration;
use std::time::Instant;

use ratatui::Frame;
use ratatui::Terminal;
use ratatui::crossterm::event::Event;
use ratatui::crossterm::event::KeyCode;
use ratatui::crossterm::event::KeyEvent;
use ratatui::crossterm::event::poll;
use ratatui::crossterm::event::read;
use ratatui::layout::Alignment;
use ratatui::prelude::Backend;
use ratatui::text::Line;
use ratatui::text::Span;
use ratatui::text::Text;
use ratatui::widgets::Block;
use ratatui::widgets::BorderType;
use ratatui::widgets::Borders;
use ratatui::widgets::List;
use ratatui::widgets::ListItem;
use ratatui::widgets::Paragraph;

use crate::events::ObserverInstance;
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

#[expect(unused)]
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

pub struct App {
    should_exit: Option<crate::Result>, // TODO: Put anyhow's type in here
    frame_no: Wrapping<u64>,
    arena: Option<Arena>,
}

impl App {
    pub fn new() -> Self {
        App { should_exit: None, arena: None, frame_no: Wrapping(0) }
    }

    fn advance(&mut self) {
        match self.arena {
            None => self.arena = Some(init_arena()),
            Some(_) => {
                todo!();
            },
        }
    }

    fn exit(&mut self, reason: crate::Result) {
        self.should_exit = Some(reason)
    }

    fn draw(&self, frame: &mut Frame) {
        let window = Block::new()
            .title(crate::APP_NAME)
            .title_alignment(Alignment::Left)
            .borders(Borders::ALL)
            .border_type(BorderType::Plain);

        let mut units = Vec::new();

        if let Some(arena) = self.arena {
            for unit in arena.combatants().values() {
                let mut unit_lines = Vec::new();

                unit_lines.push(Span::from(unit.name()));
                unit_lines
                    .push(Span::from(format!("{}HP", unit.resources().life)));

                units.push(
                    Paragraph::new(Line::from(unit_lines)).block(
                        Block::new()
                            .borders(Borders::ALL)
                            .border_type(BorderType::Plain),
                    ),
                );
            }
        }
        
        // So yeah
        // uhm
        // How do you stack boxes?
        // How tf does anyone get anything done with ratatui?
        // I thought it was more high-level than incrementing a row coord yourself
        let text = Paragraph::new()
            List::new(units.into_iter().map(|p| ListItem::new(p)).collect());

        let widget = Paragraph::new("Hello, world!").block(window);
        frame.render_widget(widget, frame.area());
    }

    fn handle_event(&mut self, event: Event) -> crate::Result {
        match event {
            Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Enter => self.advance(),
                KeyCode::F(8) => self.exit(Ok(())),
                _ => {},
            },
            _ => {},
        }
        Ok(())
    }

    fn try_read_events(&mut self) -> crate::Result {
        const MAX_DELAY: Duration = Duration::from_millis(1);
        loop {
            let Ok(true) = poll(MAX_DELAY) else { break };
            self.handle_event(read()?)?;
        }
        Ok(())
    }

    pub fn run(&mut self, term: &mut Terminal<impl Backend>) -> crate::Result {
        loop {
            self.try_read_events()?;
            if let Some(reason) = self.should_exit.take() {
                return reason;
            }
            term.draw(|f| self.draw(f))?;
            self.frame_no += 1;
        }
    }
}
