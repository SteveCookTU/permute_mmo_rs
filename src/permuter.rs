use crate::generation::{spawn_generator, EntityResult, SpawnType};
use crate::permutation::{default_criteria, Advance, AdvanceType, PermuteMeta};
use crate::util::{calculations, SpawnInfo};
use crate::{SpawnState, Xoroshiro};
use std::cell::RefCell;
use std::rc::Rc;

pub fn permute(
    spawner: Rc<RefCell<SpawnInfo>>,
    seed: u64,
    max_depth: usize,
    criteria: Option<fn(&EntityResult, &[Advance]) -> bool>,
) -> PermuteMeta {
    let mut info = PermuteMeta {
        spawner,
        max_depth,
        criteria: criteria.unwrap_or(default_criteria),
        results: vec![],
        advances: vec![],
    };

    let state = info.spawner.borrow_mut().get_starting_state();
    let table = info.spawner.borrow().set.table;

    permute_recursion(&mut info, table, seed, state);

    info
}

fn permute_recursion(meta: &mut PermuteMeta, table: u64, seed: u64, state: SpawnState) {
    if state.count != 0 {
        permute_outbreak(meta, table, seed, state);
        return;
    }

    let (can_continue, next) = meta.attempt_next_wave();
    if !can_continue {
        return;
    }

    permute_next_table(meta, next, seed, state);

    let allow_ghosts = meta.spawner.borrow().allow_ghosts();

    if allow_ghosts && state.can_add_ghosts() {
        permute_add_ghosts(meta, seed, table, state);
    }
}

fn permute_outbreak(meta: &mut PermuteMeta, table: u64, seed: u64, state: SpawnState) {
    let (reseed, new_state) = update_respawn(meta, table, seed, state);
    continue_permute(meta, table, reseed, new_state);
}

pub fn update_respawn(
    meta: &mut PermuteMeta,
    table: u64,
    seed: u64,
    state: SpawnState,
) -> (u64, SpawnState) {
    if state.count == 0 {
        return (seed, state);
    }
    let (empty, respawn, ghosts) = state.get_respawn_info();
    let only_one_alpha = meta.spawner.borrow().no_multi_alpha();
    let result = generate_spawns(
        meta,
        table,
        seed,
        empty,
        ghosts,
        state.alive_alpha,
        only_one_alpha,
    );
    let new_state = state.add(
        respawn,
        result.alpha,
        result.aggressive,
        result.skittish,
        result.oblivious,
    );
    (result.seed, new_state)
}

fn continue_permute(meta: &mut PermuteMeta, table: u64, seed: u64, state: SpawnState) {
    let spawner = meta.spawner.clone();
    if spawner.borrow().spawn_type == SpawnType::Regular {
        let count_seed = spawner.borrow().count.count_seed;
        let can_spawn_more = spawner.borrow().count.can_spawn_more(state.alive());
        if can_spawn_more {
            meta.start(Advance {
                advance_type: AdvanceType::RG,
                raw: true,
            });
            permute_recursion(meta, table, seed, state);
            meta.end();
            spawner.borrow_mut().count.count_seed = count_seed;
        }

        for i in 1..=state.alive() {
            let step = AdvanceType::A1 as usize + (i - 1);
            meta.start(Advance {
                advance_type: AdvanceType::from(step),
                raw: true,
            });
            let new_state = state.knockout_any(i);
            permute_recursion(meta, table, seed, new_state);
            meta.end();
            spawner.borrow_mut().count.count_seed = count_seed;
        }

        return;
    }

    if state.count == 0 {
        permute_recursion(meta, table, seed, state);
        return;
    }

    if state.alive_aggressive != 0 {
        for i in 1..=state.alive_aggressive {
            let step = AdvanceType::A1 as usize + (i - 1);
            meta.start(Advance {
                advance_type: AdvanceType::from(step),
                raw: true,
            });
            let new_state = state.knockout_aggressive(i);
            permute_recursion(meta, table, seed, new_state);
            meta.end();
        }
    }

    if state.alive_oblivious != 0 {
        for i in 0..=state.alive_aggressive {
            let step = AdvanceType::O1 as usize + i;
            meta.start(Advance {
                advance_type: AdvanceType::from(step),
                raw: true,
            });
            let new_state = state.knockout_oblivious(i + 1);
            permute_recursion(meta, table, seed, new_state);
            meta.end();
        }
    }

    if state.alive_beta != 0 {
        for i in 0..=state.alive_aggressive {
            let step = AdvanceType::B1 as usize + i;
            meta.start(Advance {
                advance_type: AdvanceType::from(step),
                raw: true,
            });
            let new_state = state.knockout_beta(i + 1);
            permute_recursion(meta, table, seed, new_state);
            meta.end();
        }
    }

    for i in 2..state.alive_beta {
        let step = AdvanceType::S2 as usize + (i - 2);
        meta.start(Advance {
            advance_type: AdvanceType::from(step),
            raw: true,
        });
        let new_state = state.scare(i);
        permute_recursion(meta, table, seed, new_state);
        meta.end();
    }
}

fn generate_spawns(
    meta: &mut PermuteMeta,
    table: u64,
    seed: u64,
    count: usize,
    ghosts: usize,
    current_alpha: usize,
    only_one_alpha: bool,
) -> GenerationResult {
    let mut alpha = 0;
    let mut aggressive = 0;
    let mut beta = 0;
    let mut oblivious = 0;
    let mut rng = Xoroshiro::new(seed);
    for i in 1..=count {
        let sub_seed = rng.next_u64();
        let alpha_seed = rng.next_u64();

        if i <= ghosts {
            continue;
        }

        let no_alpha = only_one_alpha && (current_alpha + alpha) != 0;
        let spawn_type = meta.spawner.borrow().spawn_type;
        if let Some(generate) =
            spawn_generator::generate(seed, i, sub_seed, alpha_seed, table, spawn_type, no_alpha)
        {
            if generate.is_alpha {
                alpha += 1;
            } else if generate.is_oblivious() {
                oblivious += 1;
            } else if generate.is_skittish() {
                beta += 1;
            } else {
                aggressive += 1;
            }
            if meta.is_result(&generate) {
                meta.add_result(generate);
            }
        }
    }
    let result = rng.next_u64();
    GenerationResult {
        seed: result,
        alpha,
        aggressive: aggressive + alpha,
        skittish: beta,
        oblivious,
    }
}

fn permute_next_table(
    meta: &mut PermuteMeta,
    next: Rc<RefCell<SpawnInfo>>,
    seed: u64,
    exist: SpawnState,
) {
    if !next.borrow().retain_existing() {
        meta.start(Advance {
            advance_type: AdvanceType::CR,
            raw: true,
        });
    }

    let current = meta.spawner.clone();

    meta.spawner = next.clone();
    let new_alive = next.borrow_mut().count.get_next_count();
    let state = if next.borrow().retain_existing() {
        exist.adjust_count(new_alive)
    } else {
        next.borrow_mut().get_starting_state()
    };

    permute_outbreak(meta, next.borrow().set.table, seed, state);

    meta.spawner = current;

    if !next.borrow().retain_existing() {
        meta.end();
    }
}

fn permute_add_ghosts(meta: &mut PermuteMeta, seed: u64, table: u64, state: SpawnState) {
    let remain = state.empty_ghost_slots();
    for i in 1..=remain {
        let step = AdvanceType::G1 as usize + (i - 1);
        meta.start(Advance {
            advance_type: AdvanceType::from(step),
            raw: true,
        });
        let new_state = state.add_ghosts(i);
        let g_seed = calculations::get_group_seed(seed, new_state.ghost);
        permute_recursion(meta, table, g_seed, new_state);
        meta.end();
    }
}

#[derive(Default, Copy, Clone)]
struct GenerationResult {
    seed: u64,
    alpha: usize,
    aggressive: usize,
    skittish: usize,
    oblivious: usize,
}
