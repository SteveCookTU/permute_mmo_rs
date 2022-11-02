use lazy_static::lazy_static;
use std::collections::HashMap;

const NONE: &str = "(Empty Area Detail)";

lazy_static! {
    pub static ref AREA_TABLE: HashMap<u64, &'static str> = {
        let mut map = HashMap::new();
        map.insert(0, NONE);
        map.insert(0xCBF29CE484222645, NONE);
        map.insert(0xE3BBEF047A645A1D, "Obsidian Fieldlands");
        map.insert(0xE3BBEC047A645504, "Crimson Mirelands");
        map.insert(0xE3BBED047A6456B7, "Cobalt Coastlands");
        map.insert(0xE3BBEA047A64519E, "Coronet Highlands");
        map.insert(0xE3BBEB047A645351, "Alabaster Icelands");
        map
    };
}
