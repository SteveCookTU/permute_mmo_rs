use crate::generation::EntityResult;
use crate::permutation::{Advance, PermuteResult};
use crate::util::SpawnInfo;
use std::cell::RefCell;
use std::rc::Rc;

pub fn default_criteria(result: &EntityResult, _advances: &[Advance]) -> bool {
    result.is_shiny && result.is_alpha
}

#[derive(Clone)]
pub struct PermuteMeta {
    pub spawner: Rc<RefCell<SpawnInfo>>,
    pub max_depth: usize,
    pub criteria: fn(&EntityResult, &[Advance]) -> bool,
    pub results: Vec<PermuteResult>,
    pub advances: Vec<Advance>,
}

impl PermuteMeta {
    pub fn has_results(&self) -> bool {
        self.results.len() != 0
    }

    pub fn attempt_next_wave(&self) -> (bool, Rc<RefCell<SpawnInfo>>) {
        let mut next = None;
        if self.advances.len() < self.max_depth && self.spawner.borrow().get_next_wave(&mut next) {
            (true, next.unwrap())
        } else {
            (false, self.spawner.clone())
        }
    }

    pub fn start(&mut self, adv: Advance) {
        self.advances.push(adv)
    }

    pub fn end(&mut self) {
        self.advances.pop();
    }

    pub fn add_result(&mut self, entity: EntityResult) {
        let steps = self.advances.clone();
        let result = PermuteResult {
            advances: steps,
            entity,
        };
        self.results.push(result)
    }

    pub fn is_result(&self, entity: &EntityResult) -> bool {
        (self.criteria)(&entity, &self.advances)
    }

    pub fn get_lines(&self) -> Vec<String> {
        let mut lines = Vec::with_capacity(self.results.len());
        for (i, result) in self.results.iter().enumerate() {
            let parent = self.find_nearest_parent_advance_result(i, &result.advances);
            let is_action_multi_result = self.is_action_multi_result(i, &result.advances);
            let has_child_chain = self.has_child_chain(i, &result.advances);
            lines.push(result.get_line(parent, is_action_multi_result, has_child_chain));
        }
        lines
    }

    pub fn has_child_chain(&self, mut index: usize, parent: &[Advance]) -> bool {
        index += 1;
        if index >= self.results.len() {
            false
        } else {
            is_subset(parent, &self.results[index].advances)
        }
    }

    pub fn is_action_multi_result(&self, index: usize, child: &[Advance]) -> bool {
        let mut count = 0;
        for result in self.results.iter().take(index).rev() {
            if Advance::sequence_eq(&result.advances, child) {
                count += 1;
            } else {
                break;
            }
        }

        for result in self.results.iter().skip(index + 1) {
            if Advance::sequence_eq(&result.advances, child) {
                count += 1;
            } else {
                break;
            }
        }

        count != 0
    }

    pub fn find_nearest_parent_advance_result(
        &self,
        index: usize,
        child: &[Advance],
    ) -> Option<PermuteResult> {
        let start = index.saturating_sub(1);
        if start == 0 {
            None
        } else {
            let nearest = self
                .results
                .iter()
                .skip(start)
                .take(start)
                .find(|z| is_subset(&z.advances, child))
                .map(|r| r.clone());
            nearest
        }
    }
}

fn is_subset(parent: &[Advance], child: &[Advance]) -> bool {
    if parent.len() >= child.len() {
        false
    } else {
        for (parent, child) in parent.iter().zip(child) {
            if parent.advance_type != child.advance_type {
                return false;
            }
        }
        true
    }
}
