use rand::Rng;

trait HitFn: 'static {
    fn did_hit<R: Rng>(rng: &mut R, chance: i16) -> bool;

    fn overflowing_chance_hit<R: Rng>(rng: &mut R, chance: i16) -> i16 {
        if chance > 100 {
            1 + Self::overflowing_chance_hit(rng, chance - 100)
        } else {
            if Self::did_hit(rng, chance) { 1 } else { 0 }
        }
    }
}

struct SingleHit;

impl HitFn for SingleHit {
    fn did_hit<R: Rng>(rng: &mut R, chance: i16) -> bool {
        rng.gen_range(0..100) < chance
    }
}

struct DoubleHit;

impl HitFn for DoubleHit {
    fn did_hit<R: Rng>(rng: &mut R, chance: i16) -> bool {
        let mut roll = || rng.gen_range(0..100);
        let average = (roll() + roll()) / 2;
        average < chance
    }
}

pub fn single_rng_hit<R: Rng>(random: &mut R, chance: i16) -> bool {
    random.gen_range(0..100) < chance
}

pub fn double_rng_hit<R: Rng>(random: &mut R, chance: i16) -> bool {
    let mut roll = || random.gen_range(0..100);
    let average = (roll() + roll()) / 2;
    average < chance
}

pub fn overflowing_chance_hit<R: Rng>(random: &mut R, mut chance: i16) -> i16 {
    let mut result = 0;
    loop {
        if chance >= 100 {
            result += 1;
            chance -= 100;
        } else {
            if single_rng_hit(random, chance) {
                result += 1;
            }
            return result;
        }
    }
}
