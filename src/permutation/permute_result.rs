use crate::generation::EntityResult;
use crate::permutation::{Advance, AdvanceType};
use std::fmt::{Debug, Formatter};

#[derive(Default, Clone)]
pub struct PermuteResult {
    pub advances: Vec<Advance>,
    pub entity: EntityResult,
}

impl Debug for PermuteResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.step_summary())
    }
}

impl PermuteResult {
    fn is_bonus(&self) -> bool {
        self.advances
            .iter()
            .any(|a| a.advance_type == AdvanceType::CR)
    }

    fn wave_index(&self) -> usize {
        self.advances
            .iter()
            .filter(|a| a.advance_type == AdvanceType::CR)
            .count()
    }

    pub fn get_line(
        &self,
        prev: Option<PermuteResult>,
        is_action_multi_result: bool,
        has_child_chain: bool,
    ) -> String {
        let steps = self.get_steps(prev.clone());
        let feasibility = self.get_feasibility(&self.advances);
        let mut line = format!(
            "* {:<37} >>> {}Spawn {} = {}{}",
            steps,
            self.get_wave_indicator(),
            self.entity.index,
            self.entity.get_summary(),
            feasibility
        );
        if prev.is_some() || has_child_chain {
            line = format!("{} ~~ Chain result!", line);
        }
        if is_action_multi_result {
            line = format!("{} ~~ Spawns multiple results!", line);
        }
        line
    }

    fn get_wave_indicator(&self) -> String {
        if !self.is_bonus() {
            "      ".to_string()
        } else {
            let wave_index = self.wave_index();
            if wave_index == 1 {
                "Bonus ".to_string()
            } else {
                format!("Wave {wave_index}")
            }
        }
    }

    fn step_summary(&self) -> String {
        format!(
            "{} {:0>16X} {}",
            self.entity.index,
            self.entity.group_seed,
            self.get_steps(None)
        )
    }

    pub fn get_steps(&self, prev: Option<PermuteResult>) -> String {
        let steps = self
            .advances
            .iter()
            .map(|a| a.get_name())
            .collect::<Vec<String>>()
            .join("|");
        if let Some(p) = prev {
            let prev_seq = p.get_steps(None);
            let repeated = ["-> "].repeat((prev_seq.len() + 2) / 3).join("");
            format!(
                "{}{}",
                repeated,
                steps.as_str()[(prev_seq.len() + 1)..].to_string()
            )
        } else {
            steps
        }
    }

    fn get_feasibility(&self, advances: &[Advance]) -> &'static str {
        if advances.iter().any(Advance::is_multi_scare) {
            if advances.iter().any(Advance::is_multi_beta) {
                return " -- Skittish: Multi scaring with aggressive!";
            }
            return " -- Skittish: Multi scaring!";
        }
        if advances.iter().any(Advance::is_multi_beta) {
            return " -- Skittish: Aggressive!";
        }

        if advances.iter().any(|a| a.advance_type == AdvanceType::B1) {
            if !advances.iter().any(Advance::is_multi_aggressive) {
                return " -- Skittish: Single advances!";
            }
            return " -- Skittish: Mostly aggressive!";
        }

        if advances.iter().any(Advance::is_multi_oblivious) {
            return " -- Oblivious: Aggressive!";
        }

        if advances.iter().any(Advance::is_multi_aggressive) {
            return "";
        }
        " -- Single Advances!"
    }
}
