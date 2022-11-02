use std::fmt::Debug;

#[derive(Default, Copy, Clone, Debug)]
pub struct SpawnState {
    pub count: usize,
    pub max_alive: usize,
    pub ghost: usize,
    pub alive_alpha: usize,
    pub alive_aggressive: usize,
    pub alive_beta: usize,
    pub alive_oblivious: usize,
    pub dead: usize,
}

impl SpawnState {
    pub fn alive(&self) -> usize {
        self.max_alive - self.dead
    }

    pub fn max_ghosts(&self) -> usize {
        self.max_alive - 1
    }

    pub fn can_add_ghosts(&self) -> bool {
        self.ghost != self.max_ghosts()
    }

    pub fn empty_ghost_slots(&self) -> usize {
        self.max_ghosts() - self.ghost
    }

    pub fn get_basic(count: usize) -> Self {
        SpawnState::get(count, count)
    }

    pub fn get(total_count: usize, alive_count: usize) -> Self {
        Self {
            count: total_count,
            max_alive: alive_count,
            ghost: 0,
            alive_alpha: 0,
            alive_aggressive: 0,
            alive_beta: 0,
            alive_oblivious: 0,
            dead: alive_count,
        }
    }

    pub fn knockout_aggressive(&self, count: usize) -> Self {
        self.remove(count, 0, 0)
    }

    pub fn knockout_beta(&self, count: usize) -> Self {
        self.remove(count - 1, 1, 0)
    }

    pub fn knockout_oblivious(&self, count: usize) -> Self {
        self.remove(count - 1, 0, 1)
    }

    pub fn knockout_any(&self, count: usize) -> Self {
        let aggro = 0.max(self.alive_aggressive.min(count));
        let beta = 0.max((self.alive_beta.saturating_sub(aggro)).min(count));
        let obli = 0.max((self.alive_beta.saturating_sub(aggro).saturating_sub(beta)).min(count));
        self.remove(aggro, beta, obli)
    }

    pub fn scare(&self, count: usize) -> Self {
        self.remove(0, count, 0)
    }

    pub fn add(
        &self,
        count: usize,
        alpha: usize,
        aggro: usize,
        beta: usize,
        oblivious: usize,
    ) -> Self {
        let n_alpha = self.alive_alpha + alpha;
        let n_aggro = self.alive_aggressive + aggro;
        let n_beta = self.alive_beta + beta;
        let n_oblivious = self.alive_oblivious + oblivious;
        debug_assert!(n_alpha <= self.max_alive);
        debug_assert!(n_aggro <= self.max_alive);
        debug_assert!(n_beta <= self.max_alive);
        debug_assert!(n_oblivious <= self.max_alive);

        let delta = aggro + beta + oblivious;
        let n_dead = self.dead - count;
        let n_ghost = n_dead;
        debug_assert!(delta > 0);
        debug_assert!(count >= delta);
        debug_assert!(n_dead < self.dead);
        debug_assert!(n_ghost < self.max_alive);
        debug_assert!(n_ghost <= self.dead);

        Self {
            count: self.count - count,
            max_alive: self.max_alive,
            ghost: n_ghost,
            alive_alpha: n_alpha,
            alive_aggressive: n_aggro,
            alive_beta: n_beta,
            alive_oblivious: n_oblivious,
            dead: n_dead,
        }
    }

    pub fn remove(&self, aggro: usize, beta: usize, oblivious: usize) -> Self {
        let n_alpha = self.alive_alpha - self.alive_alpha.min(aggro);
        let n_aggro = self.alive_aggressive - aggro;
        let n_beta = self.alive_beta - beta;
        let n_oblivious = self.alive_oblivious - oblivious;
        debug_assert!(n_alpha <= self.max_alive);
        debug_assert!(n_aggro <= self.max_alive);
        debug_assert!(n_beta <= self.max_alive);
        debug_assert!(n_oblivious <= self.max_alive);

        let delta = aggro + beta + oblivious;
        let n_dead = self.dead + delta;
        debug_assert!(n_dead <= self.max_alive);
        debug_assert!(delta > 0);

        Self {
            count: self.count,
            max_alive: self.max_alive,
            ghost: self.ghost,
            alive_alpha: n_alpha,
            alive_aggressive: n_aggro,
            alive_beta: n_beta,
            alive_oblivious: n_oblivious,
            dead: n_dead,
        }
    }

    pub fn adjust_count(&self, new_alive: usize) -> Self {
        let max_alive = new_alive.max(self.alive());
        let new_count = max_alive.saturating_sub(self.alive());
        let new_dead = max_alive.saturating_sub(self.alive());
        Self {
            count: new_count,
            max_alive,
            ghost: self.ghost,
            alive_alpha: self.alive_alpha,
            alive_aggressive: self.alive_aggressive,
            alive_beta: self.alive_beta,
            alive_oblivious: self.alive_oblivious,
            dead: new_dead,
        }
    }

    pub fn add_ghosts(&self, count: usize) -> Self {
        SpawnState {
            count: self.count,
            max_alive: self.max_alive,
            ghost: self.ghost + count,
            alive_alpha: 0,
            alive_aggressive: 0,
            alive_beta: 0,
            alive_oblivious: 0,
            dead: self.dead + count,
        }
    }

    pub fn get_respawn_info(&self) -> (usize, usize, usize) {
        let empty_slots = self.dead;
        let respawn = self.count.min(empty_slots);
        let ghosts = empty_slots - respawn;
        debug_assert!(respawn != 0 || self.dead == 0);
        (empty_slots, respawn, ghosts)
    }

    pub fn get_state(&self) -> String {
        let mut ctr = 0;
        let mut result = vec![char::default(); self.max_alive];
        for _ in 0..self.alive_alpha {
            result[ctr] = 'a';
            ctr += 1;
        }
        for _ in 0..(self.alive_aggressive - self.alive_alpha) {
            result[ctr] = 'A';
            ctr += 1;
        }
        for _ in 0..self.alive_beta {
            result[ctr] = 'B';
            ctr += 1;
        }
        for _ in 0..self.alive_oblivious {
            result[ctr] = 'O';
            ctr += 1;
        }
        for _ in 0..self.ghost {
            result[ctr] = '~';
            ctr += 1;
        }
        for _ in 0..(self.dead - self.ghost) {
            result[ctr] = 'X';
            ctr += 1;
        }
        while ctr != result.len() {
            result[ctr] = '?';
            ctr += 1;
        }

        result.into_iter().collect()
    }
}
