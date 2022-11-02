pub mod area_util;
pub mod behavior_util;
pub mod calculations;
pub mod json_decoder;
pub mod permute_dump;
mod spawn_info;

use lazy_static::lazy_static;
use pkhex_rs::game_strings::SPECIES_EN;
pub use spawn_info::*;
use std::collections::HashMap;

lazy_static! {
    pub static ref SPECIES_DICT: HashMap<&'static str, u16> = {
        let mut map = HashMap::with_capacity(SPECIES_EN.len());
        for (i, name) in SPECIES_EN.iter().enumerate() {
            map.insert(*name, i as u16);
        }
        map
    };
}
