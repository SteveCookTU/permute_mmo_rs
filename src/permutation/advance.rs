use crate::permutation::PermuteMeta;
use crate::util::calculations;
use crate::{permuter, SpawnState};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
#[repr(u8)]
pub enum AdvanceType {
    RG,
    CR,
    A1,
    A2,
    A3,
    A4,
    B1,
    B2,
    B3,
    B4,
    O1,
    O2,
    O3,
    O4,
    S2,
    S3,
    S4,
    G1,
    G2,
    G3,
}

impl From<usize> for AdvanceType {
    fn from(num: usize) -> Self {
        match num {
            0 => AdvanceType::RG,
            1 => AdvanceType::CR,
            2 => AdvanceType::A1,
            3 => AdvanceType::A2,
            4 => AdvanceType::A3,
            5 => AdvanceType::A4,
            6 => AdvanceType::B1,
            7 => AdvanceType::B2,
            8 => AdvanceType::B3,
            9 => AdvanceType::B4,
            10 => AdvanceType::O1,
            11 => AdvanceType::O2,
            12 => AdvanceType::O3,
            13 => AdvanceType::O4,
            14 => AdvanceType::S2,
            15 => AdvanceType::S3,
            16 => AdvanceType::S4,
            17 => AdvanceType::G1,
            18 => AdvanceType::G2,
            _ => AdvanceType::G3,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Advance {
    pub advance_type: AdvanceType,
    pub raw: bool,
}

impl ToString for AdvanceType {
    fn to_string(&self) -> String {
        match self {
            AdvanceType::RG => "RG".to_string(),
            AdvanceType::CR => "CR".to_string(),
            AdvanceType::A1 => "A1".to_string(),
            AdvanceType::A2 => "A2".to_string(),
            AdvanceType::A3 => "A3".to_string(),
            AdvanceType::A4 => "A4".to_string(),
            AdvanceType::B1 => "B1".to_string(),
            AdvanceType::B2 => "B2".to_string(),
            AdvanceType::B3 => "B3".to_string(),
            AdvanceType::B4 => "B4".to_string(),
            AdvanceType::O1 => "O1".to_string(),
            AdvanceType::O2 => "O2".to_string(),
            AdvanceType::O3 => "O3".to_string(),
            AdvanceType::O4 => "O4".to_string(),
            AdvanceType::S2 => "S2".to_string(),
            AdvanceType::S3 => "S3".to_string(),
            AdvanceType::S4 => "S4".to_string(),
            AdvanceType::G1 => "G1".to_string(),
            AdvanceType::G2 => "G2".to_string(),
            AdvanceType::G3 => "G3".to_string(),
        }
    }
}

impl Advance {
    pub fn get_name(&self) -> String {
        if self.raw {
            self.advance_type.to_string()
        } else {
            self.humanize()
        }
    }

    pub fn sequence_eq(a: &[Advance], b: &[Advance]) -> bool {
        a.len() == b.len()
            && a.iter()
                .zip(b.iter())
                .all(|(a, b)| a.advance_type == b.advance_type)
    }

    pub fn run_forwards(meta: &mut PermuteMeta, advances: &[Advance], seed: u64) -> Vec<SpawnStep> {
        let mut steps = Vec::new();
        let spawner = meta.spawner.clone();
        let state = spawner.borrow_mut().get_starting_state();
        let table = meta.spawner.borrow().set.table;
        let (mut seed, mut state) = permuter::update_respawn(meta, table, seed, state);
        steps.push(SpawnStep {
            step: Advance {
                advance_type: AdvanceType::RG,
                raw: true,
            },
            state,
            seed,
            count_seed: meta.spawner.borrow().count.count_seed,
        });

        for advance in advances {
            meta.start(*advance);
            if meta.spawner.borrow().retain_existing() {
                let count = advance.advance_count();
                if count != 0 {
                    state = state.knockout_any(count);
                }

                let new_alive = meta.spawner.borrow_mut().count.get_next_count();
                state = state.adjust_count(new_alive);
                steps.push(SpawnStep {
                    step: *advance,
                    state,
                    seed,
                    count_seed: meta.spawner.borrow().count.count_seed,
                })
            } else if advance.advance_type == AdvanceType::CR {
                let mut next = None;
                if !meta.spawner.borrow_mut().get_next_wave(&mut next) {
                    panic!("No next spawner available!");
                }
                if let Some(next) = next {
                    meta.spawner = next.clone();
                    state = next.borrow_mut().get_starting_state();
                    steps.push(SpawnStep {
                        step: *advance,
                        state,
                        seed,
                        count_seed: meta.spawner.borrow().count.count_seed,
                    });
                } else if advance.advance_type as usize >= AdvanceType::G1 as usize {
                    let count = advance.advance_count();
                    state = state.add_ghosts(count);
                    seed = calculations::get_group_seed(seed, state.ghost);
                    steps.push(SpawnStep {
                        step: *advance,
                        state,
                        seed,
                        count_seed: meta.spawner.borrow().count.count_seed,
                    });
                    continue;
                } else {
                    state = advance.advance_state(state);
                    steps.push(SpawnStep {
                        step: *advance,
                        state,
                        seed,
                        count_seed: meta.spawner.borrow().count.count_seed,
                    });
                }

                if state.count != 0 {
                    let table = meta.spawner.borrow().set.table;
                    (seed, state) = permuter::update_respawn(meta, table, seed, state);
                }
                steps.push(SpawnStep {
                    step: Advance {
                        advance_type: AdvanceType::RG,
                        raw: true,
                    },
                    state,
                    seed,
                    count_seed: meta.spawner.borrow().count.count_seed,
                })
            }
        }
        steps
    }

    fn humanize(&self) -> String {
        match self.advance_type {
            AdvanceType::CR => "Clear Remaining".to_string(),
            AdvanceType::A1 => "1 Aggressive".to_string(),
            AdvanceType::A2 => "2 Aggressive".to_string(),
            AdvanceType::A3 => "3 Aggressive".to_string(),
            AdvanceType::A4 => "4 Aggressive".to_string(),
            AdvanceType::B1 => "1 Beta".to_string(),
            AdvanceType::B2 => "1 Beta + 1 Aggressive".to_string(),
            AdvanceType::B3 => "1 Beta + 2 Aggressive".to_string(),
            AdvanceType::B4 => "1 Beta + 3 Aggressive".to_string(),
            AdvanceType::O1 => "1 Oblivious".to_string(),
            AdvanceType::O2 => "1 Oblivious + 1 Aggressive".to_string(),
            AdvanceType::O3 => "1 Oblivious + 2 Aggressive".to_string(),
            AdvanceType::O4 => "1 Oblivious + 3 Aggressive".to_string(),
            AdvanceType::S2 => "Multi Scare 2 + Leave".to_string(),
            AdvanceType::S3 => "Multi Scare 3 + Leave".to_string(),
            AdvanceType::S4 => "Multi Scare 4 + Leave".to_string(),
            AdvanceType::G1 => "De-spawn 1 + Leave".to_string(),
            AdvanceType::G2 => "De-spawn 2 + Leave".to_string(),
            AdvanceType::G3 => "De-spawn 3 + Leave".to_string(),
            _ => panic!("Invalid advance type to humanize!"),
        }
    }

    pub fn advance_count(&self) -> usize {
        match self.advance_type {
            AdvanceType::A1 | AdvanceType::B1 | AdvanceType::O1 | AdvanceType::G1 => 1,
            AdvanceType::A2
            | AdvanceType::B2
            | AdvanceType::O2
            | AdvanceType::S2
            | AdvanceType::G2 => 2,
            AdvanceType::A3
            | AdvanceType::B3
            | AdvanceType::O3
            | AdvanceType::S3
            | AdvanceType::G3 => 3,
            AdvanceType::A4 | AdvanceType::B4 | AdvanceType::O4 | AdvanceType::S4 => 4,
            _ => 0,
        }
    }

    pub fn is_multi_aggressive(&self) -> bool {
        self.advance_type == AdvanceType::A2
            || self.advance_type == AdvanceType::A3
            || self.advance_type == AdvanceType::A4
    }

    pub fn is_multi_scare(&self) -> bool {
        self.advance_type == AdvanceType::S2
            || self.advance_type == AdvanceType::S3
            || self.advance_type == AdvanceType::S4
    }

    pub fn is_multi_beta(&self) -> bool {
        self.advance_type == AdvanceType::B2
            || self.advance_type == AdvanceType::B3
            || self.advance_type == AdvanceType::B4
    }

    pub fn is_multi_oblivious(&self) -> bool {
        self.advance_type == AdvanceType::O2
            || self.advance_type == AdvanceType::O3
            || self.advance_type == AdvanceType::O4
    }

    pub fn get_removals(&self) -> (usize, usize, usize) {
        let count = self.advance_count();
        if self.is_multi_aggressive() || self.advance_type == AdvanceType::A1 {
            (count, 0, 0)
        } else if self.is_multi_beta() || self.advance_type == AdvanceType::B1 {
            (count - 1, 1, 0)
        } else if self.is_multi_scare() {
            (0, count, 0)
        } else if self.is_multi_oblivious() || self.advance_type == AdvanceType::O1 {
            (count - 1, 0, 1)
        } else {
            panic!("Invalid advance type for removals")
        }
    }

    pub fn advance_state(&self, state: SpawnState) -> SpawnState {
        let (aggro, beta, oblivious) = self.get_removals();
        state.remove(aggro, beta, oblivious)
    }
}

#[derive(Copy, Clone)]
pub struct SpawnStep {
    pub step: Advance,
    pub state: SpawnState,
    pub seed: u64,
    pub count_seed: u64,
}

impl SpawnStep {
    pub fn step_summary(&self) -> String {
        format!(
            "{} {} {} {:0>16X} {:0>16X}",
            self.step.advance_type.to_string(),
            self.state.get_state(),
            self.state.count,
            self.seed,
            self.count_seed
        )
    }
}
