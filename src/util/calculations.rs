use crate::permutation::Advance;
use crate::Xoroshiro;

pub fn get_group_seed_from_advances(group_seed: u64, advances: &[Advance]) -> u64 {
    let mut seed = get_group_seed(group_seed, 4);
    for advance in advances {
        let count = advance.advance_count();
        let mut rng = Xoroshiro::new(seed);
        for _ in 0..count {
            rng.next_u64();
            rng.next_u64();
        }
        seed = rng.next_u64();
    }

    seed
}

pub fn get_group_seed(seed: u64, count: usize) -> u64 {
    let mut rng = Xoroshiro::new(seed);
    for _ in 0..count {
        rng.next_u64();
        rng.next_u64();
    }
    rng.next_u64()
}

pub fn get_generate_seed(group_seed: u64, spawn_index: usize) -> (u64, u64) {
    let mut rng = Xoroshiro::new(group_seed);
    for i in 1..=spawn_index {
        let sub_seed = rng.next_u64();
        let alpha = rng.next_u64();
        if i == spawn_index {
            return (sub_seed, alpha);
        }
    }
    panic!("Spawn index must not be 0")
}

pub fn get_entity_seed(group_seed: u64, spawn_index: usize) -> u64 {
    let mut rng = Xoroshiro::new(group_seed);
    for i in 1..=spawn_index {
        let sub_seed = rng.next_u64();
        rng.next_u64();

        if i != spawn_index {
            continue;
        }

        let mut poke = Xoroshiro::new(sub_seed);
        poke.next_u64();
        return poke.next_u64();
    }
    panic!("Spawn index must not be 0")
}
