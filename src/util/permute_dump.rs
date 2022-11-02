use crate::permutation::{AdvanceType, PermuteMeta, PermuteResult};
use std::collections::BTreeMap;

pub fn dump(meta: PermuteMeta) -> Vec<String> {
    let results = meta.results;
    let mut groups: BTreeMap<usize, Vec<PermuteResult>> = BTreeMap::new();
    results.into_iter().for_each(|result| {
        groups
            .entry(result.advances.len())
            .or_insert(vec![])
            .push(result)
    });

    let mut lines = Vec::new();

    for group in groups {
        let step = group.0;
        let entities = group.1;
        let first = entities.first().unwrap();
        let adv = if step == 0 {
            AdvanceType::RG
        } else {
            first.advances[step - 1].advance_type
        };
        for line in get_lines(step, adv, &entities) {
            lines.push(line)
        }
    }
    lines
}

fn get_lines(step: usize, adv: AdvanceType, entities: &[PermuteResult]) -> Vec<String> {
    let mut lines = Vec::with_capacity(3 + entities.len() * 15 + entities.len() + 1);

    lines.push("===================".to_string());
    lines.push(format!("Step {step}: {}", adv.to_string()));
    lines.push(
        entities[0]
            .advances
            .iter()
            .map(|a| a.advance_type.to_string())
            .collect::<Vec<String>>()
            .join("|"),
    );
    for entity in entities {
        for line in entity.entity.get_lines() {
            lines.push(line);
        }
        lines.push(String::new());
    }
    lines.push(String::new());

    lines
}
