use rand::Rng;

pub fn single_rng_hit<R: Rng>(random: &mut R, chance: i16) -> bool {
    random.gen_range(0..100) < chance
}

pub fn double_rng_hit<R: Rng>(random: &mut R, chance: i16) -> bool {
    let mut roll = || random.gen_range(0..100);
    let average = (roll() + roll()) / 2;
    average < chance
}

pub fn increment_hit<R: Rng>(random: &mut R, mut chance: i16) -> i16 {
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
