use crate::generation::SpawnType;
use crate::structure::{MassOutbreakSpawner8a, MassiveOutbreakSpawner8a};
use crate::{SpawnState, Xoroshiro};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Clone, Debug)]
pub struct SpawnInfo {
    pub count: SpawnCount,
    pub set: SpawnSet,
    pub spawn_type: SpawnType,
    pub parent: Weak<RefCell<SpawnInfo>>,
    pub next: Option<Rc<RefCell<SpawnInfo>>>,
}

impl SpawnInfo {
    const MMO: SpawnCount = SpawnCount {
        max_alive: 4,
        min_alive: 4,
        count_seed: 0,
    };
    const OUTBREAK: SpawnCount = SpawnCount {
        max_alive: 4,
        min_alive: 4,
        count_seed: 0,
    };

    pub fn no_multi_alpha(&self) -> bool {
        self.spawn_type == SpawnType::Regular || self.spawn_type == SpawnType::Outbreak
    }

    pub fn allow_ghosts(&self) -> bool {
        self.spawn_type != SpawnType::Regular
    }

    pub fn retain_existing(&self) -> bool {
        self.spawn_type == SpawnType::Regular
    }

    pub fn get_next_wave(&self, next: &mut Option<Rc<RefCell<SpawnInfo>>>) -> bool {
        if let Some(inner_next) = self.next.as_ref() {
            *next = Some(inner_next.clone());
            true
        } else {
            false
        }
    }

    pub fn get_mmo(
        base_table: u64,
        base_count: usize,
        bonus_table: u64,
        bonus_count: usize,
    ) -> Rc<RefCell<Self>> {
        Rc::new_cyclic(|me| {
            RefCell::new(Self {
                count: SpawnInfo::MMO,
                set: SpawnSet {
                    table: base_table,
                    count: base_count,
                },
                spawn_type: SpawnType::MMO,
                parent: Weak::new(),
                next: Some(Rc::new(RefCell::new(SpawnInfo {
                    count: SpawnInfo::MMO,
                    set: SpawnSet {
                        table: bonus_table,
                        count: bonus_count,
                    },
                    spawn_type: SpawnType::MMO,
                    parent: me.clone(),
                    next: None,
                }))),
            })
        })
    }

    pub fn get_starting_state(&mut self) -> SpawnState {
        if self.spawn_type == SpawnType::Regular {
            SpawnState::get_basic(self.count.get_next_count())
        } else {
            SpawnState::get(self.set.count, self.count.max_alive)
        }
    }

    fn get_bonus_chain(
        spawner: MassiveOutbreakSpawner8a,
        parent: Weak<RefCell<Self>>,
    ) -> Option<Rc<RefCell<Self>>> {
        if !spawner.has_bonus() {
            None
        } else {
            Some(Rc::new(RefCell::new(SpawnInfo {
                count: SpawnInfo::MMO,
                set: SpawnSet {
                    table: spawner.bonus_table,
                    count: spawner.bonus_count as usize,
                },
                spawn_type: SpawnType::MMO,
                parent,
                next: None,
            })))
        }
    }

    pub fn get_mo(table: u64, count: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            count: SpawnInfo::OUTBREAK,
            set: SpawnSet { table, count },
            spawn_type: SpawnType::Outbreak,
            parent: Weak::new(),
            next: None,
        }))
    }

    pub fn get_loop(count: SpawnCount, set: SpawnSet, spawn_type: SpawnType) -> Rc<RefCell<Self>> {
        Rc::new_cyclic(|parent| {
            RefCell::new(Self {
                count,
                set,
                spawn_type,
                parent: parent.clone(),
                next: None,
            })
        })
    }
}

impl From<MassiveOutbreakSpawner8a> for Rc<RefCell<SpawnInfo>> {
    fn from(spawner: MassiveOutbreakSpawner8a) -> Self {
        Rc::new_cyclic(|me| {
            RefCell::new(SpawnInfo {
                count: SpawnInfo::MMO,
                set: SpawnSet {
                    table: spawner.base_table,
                    count: spawner.base_count as usize,
                },
                spawn_type: SpawnType::MMO,
                parent: Weak::new(),
                next: SpawnInfo::get_bonus_chain(spawner, me.clone()),
            })
        })
    }
}

impl From<MassOutbreakSpawner8a> for Rc<RefCell<SpawnInfo>> {
    fn from(spawner: MassOutbreakSpawner8a) -> Self {
        Rc::new(RefCell::new(SpawnInfo {
            count: SpawnInfo::OUTBREAK,
            set: SpawnSet {
                table: spawner.display_species as u64,
                count: spawner.base_count as usize,
            },
            spawn_type: SpawnType::Outbreak,
            parent: Weak::new(),
            next: None,
        }))
    }
}

pub fn get_summary(info: &Rc<RefCell<SpawnInfo>>, prefix: &str) -> String {
    let summary = format!("{prefix}{:?}", info);
    if let Some(x) = &info.borrow().parent.upgrade() {
        if Rc::ptr_eq(info, x) {
            format!("{summary} REPEATING.")
        } else {
            format!("{summary}\n{}", get_summary(&x, prefix))
        }
    } else {
        summary
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SpawnSet {
    pub table: u64,
    pub count: usize,
}

#[derive(Copy, Clone, Debug)]
pub struct SpawnCount {
    pub max_alive: usize,
    pub min_alive: usize,
    pub count_seed: u64,
}

impl SpawnCount {
    pub fn is_fixed_count(&self) -> bool {
        self.min_alive == 0 || self.min_alive == self.max_alive
    }

    pub fn get_next_count(&mut self) -> usize {
        if self.is_fixed_count() {
            self.max_alive
        } else {
            let mut rand = Xoroshiro::new(self.count_seed);
            let delta = self.max_alive - self.min_alive;
            let result = self.min_alive + rand.next_max((delta + 1) as u64) as usize;
            self.count_seed = rand.next_u64();
            result
        }
    }

    fn peek_next_count(&self) -> usize {
        let mut rand = Xoroshiro::new(self.count_seed);
        let delta = self.max_alive - self.min_alive;
        self.min_alive + rand.next_max((delta + 1) as u64) as usize
    }

    pub fn can_spawn_more(&self, current_max_alive: usize) -> bool {
        if self.is_fixed_count() {
            false
        } else {
            let next_max_alive = self.peek_next_count();
            if next_max_alive > current_max_alive {
                true
            } else {
                next_max_alive == current_max_alive && next_max_alive != self.max_alive
            }
        }
    }
}
