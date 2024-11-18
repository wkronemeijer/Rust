use std::time::Instant;

use srpg_arena::game::Arena;
use srpg_arena::game::PrintLnDelegate;
use srpg_arena::game::Stats;
use srpg_arena::game::Unit;

fn main() {
    let mut arena = Arena::new();
    let mut observer = PrintLnDelegate;

    let john = {
        let mut stats = Stats::default();

        stats.maximum_life = 20;
        stats.phys_damage = 9;
        stats.phys_defense = 4;
        stats.attack_speed = 3;
        stats.hit = 90;
        stats.avoid = 35;

        stats.crit = 15;

        Unit::new("Aerith", stats)
    };

    let jimmy = {
        let mut stats = Stats::default();

        stats.maximum_life = 30;
        stats.phys_damage = 11;
        stats.phys_defense = 5;
        stats.attack_speed = 3;
        stats.hit = 85;
        stats.avoid = 15;

        stats.crit = 5;

        Unit::new("Bob", stats)
    };

    arena.add(john);
    arena.add(jimmy);

    let before = Instant::now();
    let result = arena.fight_to_the_death(&mut observer);
    let after = Instant::now();

    match result.victor {
        Some(unit) => println!("{} is victorious!", unit.name()),
        None => println!("Everyone died..."),
    }

    let millis = (after - before).as_millis();
    println!("(completed in {}ms)", millis)
}
