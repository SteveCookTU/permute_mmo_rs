use crate::util::{behavior_util, SPECIES_DICT};
use serde::Deserialize;
use std::collections::HashMap;
use std::str::FromStr;

pub fn get_map(json: &str) -> HashMap<u64, Vec<SlotDetail>> {
    let obj = serde_json::from_str::<HashMap<String, Vec<SlotDetail>>>(json).unwrap();
    let mut result = HashMap::with_capacity(obj.len());
    for (key, mut value) in obj {
        if let Ok(hash) = u64::from_str_radix(&key.as_str()[2..], 16) {
            for slot in value.iter_mut() {
                slot.set_species();
            }
            result.insert(hash, value);
        }
    }
    result
}

#[derive(Deserialize, Default, Clone, Debug)]
pub struct SlotDetail {
    #[serde(rename = "slot")]
    pub rate: usize,
    pub name: String,
    #[serde(rename = "alpha")]
    pub is_alpha: bool,
    pub level: Option<[usize; 2]>,
    #[serde(rename = "ivs")]
    pub flawless_ivs: usize,
    #[serde(skip_deserializing)]
    pub species: u16,
    #[serde(skip_deserializing)]
    pub form: u16,
}

impl SlotDetail {
    pub fn min_level(&self) -> usize {
        if let Some(level) = self.level.as_ref() {
            level[0]
        } else {
            0
        }
    }

    pub fn max_level(&self) -> usize {
        if let Some(level) = self.level.as_ref() {
            level[1]
        } else {
            0
        }
    }

    pub fn is_skittish(&self) -> bool {
        behavior_util::SKITTISH.contains(&self.species)
    }

    pub fn set_species(&mut self) {
        let mut species;
        if let Some(dash) = self.name.as_bytes().iter().position(|c| *c == b'-') {
            if let Ok(form) = u16::from_str(&self.name.as_str()[(dash + 1)..]) {
                self.form = form;
                species = self.name[..dash].to_string();
            } else {
                panic!("Invalid number for form")
            }
        } else {
            species = self.name.clone();
        }

        if species.as_str() == "MimeJr." {
            species = String::from("Mime Jr.");
        }
        if species.as_str() == "Mr.Mime" {
            species = String::from("Mr. Mime");
        }

        if let Some(species) = SPECIES_DICT.get(species.as_str()) {
            self.species = *species;
        } else {
            panic!("No species found!")
        }
    }
}
