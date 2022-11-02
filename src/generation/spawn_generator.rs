use crate::generation::{EntityResult, SpawnType};
use crate::util::json_decoder::{get_map, SlotDetail};
use crate::Xoroshiro;
use lazy_static::lazy_static;
use pkhex_rs::game_strings::SPECIES_EN;
use pkhex_rs::{
    personal_table, PersonalInfo, Species, RATIO_MAGIC_FEMALE, RATIO_MAGIC_GENDERLESS,
    RATIO_MAGIC_MALE,
};
use std::collections::HashMap;

const SLOTS_RAW: &str = include_str!("../../resources/mmo_es.json");

lazy_static! {
    pub static ref SLOT_MAP: HashMap<u64, Vec<SlotDetail>> = get_map(SLOTS_RAW);
}

pub fn generate(
    group_seed: u64,
    index: usize,
    seed: u64,
    alpha_seed: u64,
    table: u64,
    spawn_type: SpawnType,
    no_alpha: bool,
) -> Option<EntityResult> {
    let mut slot_rng = Xoroshiro::new(seed);

    let slots = get_slots(table);

    let slot_sum = get_slot_sum(&slots, no_alpha);
    if slot_sum == 0.0 {
        return None;
    }

    let slot_roll = slot_rng.next_f32(slot_sum, 0.0);
    let slot = get_slot(&slots, slot_roll, no_alpha);
    let gen_seed = slot_rng.next_u64();
    let level = get_level(&slot, slot_rng);
    let gt = personal_table::LA
        .get_form_entry(slot.species as usize, slot.form as usize)
        .get_gender();

    let shiny_rolls = spawn_type as usize;
    let flawless_ivs = slot.flawless_ivs;
    let mut result = EntityResult {
        species: slot.species,
        form: slot.form,
        level,
        ec: 0,
        fake_tid: 0,
        pid: 0,
        shiny_xor: 0,
        roll_count_used: 0,
        is_alpha: slot.is_alpha,
        ability: 0,
        gender: 0,
        nature: 0,
        height: 0,
        group_seed,
        index,
        slot_seed: seed,
        gen_seed,
        alpha_seed,
        slot_roll,
        ivs: [0; 6],
        roll_count_allowed: 0,
        is_shiny: false,
        weight: 0,
        slot,
    };

    generate_pokemon(&mut result, gen_seed, shiny_rolls, flawless_ivs, gt);

    Some(result)
}

fn get_slots(table: u64) -> Vec<SlotDetail> {
    if table > 1000 {
        SLOT_MAP.get(&table).unwrap().clone()
    } else {
        let species = table as u16;
        get_fake_outbreak(species)
    }
}

fn get_fake_outbreak(species: u16) -> Vec<SlotDetail> {
    let mut name = SPECIES_EN[species as usize].to_string();
    if species == Species::Basculin as u16 {
        name = format!("{}-2", name);
    }

    let mut value = vec![
        SlotDetail {
            rate: 100,
            name: name.clone(),
            is_alpha: false,
            level: Some([0, 1]),
            flawless_ivs: 0,
            species: 0,
            form: 0,
        },
        SlotDetail {
            rate: 1,
            name,
            is_alpha: true,
            level: Some([0, 1]),
            flawless_ivs: 3,
            species: 0,
            form: 0,
        },
    ];
    for slot in value.iter_mut() {
        slot.set_species();
    }

    value
}

fn get_level(slot: &SlotDetail, mut slot_rng: Xoroshiro) -> usize {
    let min = slot.min_level();
    let max = slot.max_level();

    let mut level = min;
    let delta = max - min;

    if delta != 0 {
        level += slot_rng.next_max((delta + 1) as u64) as usize;
    }
    level
}

fn get_slot_sum(slots: &[SlotDetail], no_alpha: bool) -> f32 {
    let mut total = 0.0;
    for slot in slots {
        if no_alpha && slot.is_alpha {
            continue;
        }
        total += slot.rate as f32;
    }
    total
}

fn get_slot(slots: &[SlotDetail], mut slot_roll: f32, no_alpha: bool) -> SlotDetail {
    for slot in slots {
        if no_alpha && slot.is_alpha {
            continue;
        }

        slot_roll -= slot.rate as f32;
        if slot_roll <= 0.0 {
            return slot.clone();
        }
    }
    panic!("Slot roll out of range of slot values")
}

pub fn generate_pokemon(
    result: &mut EntityResult,
    seed: u64,
    shiny_rolls: usize,
    flawless: usize,
    gender_ratio: usize,
) {
    let mut rng = Xoroshiro::new(seed);
    result.ec = rng.next_max(0xFFFFFFFF) as u32;
    result.fake_tid = rng.next_max(0xFFFFFFFF) as u32;

    let mut pid;
    let mut ctr = 0;
    loop {
        ctr += 1;
        pid = rng.next_max(0xFFFFFFFF) as u32;
        let shiny_xor = get_shiny_xor(pid, result.fake_tid);
        let is_shiny = shiny_xor < 16;
        result.is_shiny = is_shiny;
        if !is_shiny {
            if ctr >= shiny_rolls {
                break;
            } else {
                continue;
            }
        }

        result.shiny_xor = shiny_xor;
        result.roll_count_used = ctr;
        result.roll_count_allowed = shiny_rolls;
        break;
    }
    result.pid = pid;
    let ivs = &mut result.ivs;

    for _ in 0..flawless {
        let mut index = rng.next_max(6) as usize;
        while ivs[index] != 0 {
            index = rng.next_max(6) as usize;
        }

        ivs[index] = 31;
    }

    for iv in ivs.iter_mut() {
        if *iv == 0 {
            *iv = rng.next_max(32) as u8;
        }
    }

    result.ability = rng.next_max(2) as u8;

    result.gender = match gender_ratio {
        i if i == RATIO_MAGIC_GENDERLESS => 2,
        i if i == RATIO_MAGIC_FEMALE => 1,
        i if i == RATIO_MAGIC_MALE => 0,
        _ => {
            if (rng.next_max(253) as usize + 1) < gender_ratio {
                1
            } else {
                0
            }
        }
    };

    result.nature = rng.next_max(25) as u8;
    result.height = if result.is_alpha {
        u8::MAX
    } else {
        (rng.next_max(0x81) + rng.next_max(0x80)) as u8
    };
    result.weight = if result.is_alpha {
        u8::MAX
    } else {
        (rng.next_max(0x81) + rng.next_max(0x80)) as u8
    };
}

#[inline(always)]
fn get_shiny_xor(pid: u32, oid: u32) -> u32 {
    let xor = pid ^ oid;
    (xor ^ (xor >> 16)) & 0xFFFF
}
