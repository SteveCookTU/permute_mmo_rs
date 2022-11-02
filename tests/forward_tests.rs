use permute_mmo_rs::generation::EntityResult;
use permute_mmo_rs::permutation::{Advance, AdvanceType, PermuteMeta, PermuteResult};
use permute_mmo_rs::permuter;
use permute_mmo_rs::util::SpawnInfo;

#[test]
pub fn first() {
    let seed = 0xA5D779D8831721FD;
    let base_count = 10;
    let bonus_count = 6;
    let spawner = SpawnInfo::get_mmo(
        0x7FA3A1DE69BD271E,
        base_count,
        0x44182B854CD3745D,
        bonus_count,
    );
    let result = permuter::permute(spawner, seed, 15, None);

    assert!(result
        .results
        .iter()
        .find(|z| z.entity.pid == 0x6f4edff0)
        .is_some());

    let first = result.results[0].clone();
    let matched = run_forwards_regenerate(seed, result, first.clone());
    assert!(matched.is_some());
    if let Some(matched) = matched {
        assert_eq!(matched.entity.slot_seed, first.entity.slot_seed);
    }
}

#[cfg(test)]
fn run_forwards_regenerate(
    seed: u64,
    mut result: PermuteMeta,
    first: PermuteResult,
) -> Option<PermuteResult> {
    let criteria = |_: &EntityResult, _: &[Advance]| -> bool { true };
    result.criteria = criteria;
    let (advances, entity_result) = (first.advances, first.entity);
    let steps = Advance::run_forwards(&mut result, &advances, seed);
    assert!(steps.len() > 0);

    result
        .results
        .iter()
        .find(|z| {
            Advance::sequence_eq(&advances, &z.advances) && entity_result.index == z.entity.index
        })
        .map(|r| r.clone())
}

#[test]
fn test_forwards() {
    let seed = 1911689355633755303u64;
    let base_count = 9;
    let bonus_count = 7;
    let spawner = SpawnInfo::get_mmo(
        0xECBF77B8F7302126,
        base_count,
        0x9D713CCF138FD43C,
        bonus_count,
    );
    let mut result = permuter::permute(spawner, seed, 15, None);
    let seq = vec![
        Advance {
            advance_type: AdvanceType::A1,
            raw: true,
        },
        Advance {
            advance_type: AdvanceType::A1,
            raw: true,
        },
        Advance {
            advance_type: AdvanceType::A2,
            raw: true,
        },
        Advance {
            advance_type: AdvanceType::A4,
            raw: true,
        },
        Advance {
            advance_type: AdvanceType::CR,
            raw: true,
        },
        Advance {
            advance_type: AdvanceType::A2,
            raw: true,
        },
        Advance {
            advance_type: AdvanceType::A2,
            raw: true,
        },
    ];

    let _temp = Advance::run_forwards(&mut result, &seq, seed);
    let expect = result
        .results
        .iter()
        .filter(|z| Advance::sequence_eq(&seq, &z.advances))
        .collect::<Vec<_>>();
    assert!(expect.first().is_some());
    let first = expect.first().unwrap();
    assert!(first.entity.is_shiny);
    assert_eq!(first.entity.index, 2);
}
