use crate::structure::MassiveOutbreakSpawnerStatus;
use no_std_io::EndianRead;

#[derive(Default, Copy, Clone, EndianRead, Debug)]
pub struct MassiveOutbreakSpawner8a {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    #[no_std_io(pad_before = 4)]
    status: u8,
    #[no_std_io(pad_before = 3)]
    pub display_species: u16,
    #[no_std_io(pad_before = 2)]
    pub display_form: u16,
    #[no_std_io(pad_before = 0x1E)]
    pub base_table: u64,
    pub bonus_table: u64,
    pub aguav_seed: u64,
    pub count_seed: u64,
    pub group_seed: u64,
    pub base_count: u8,
    #[no_std_io(pad_before = 3)]
    pub spawned_count: u32,
    pub spawner_name: u64,
    #[no_std_io(pad_before = 4)]
    pub bonus_count: u8,
}

impl MassiveOutbreakSpawner8a {
    pub const SIZE: usize = 0x90;

    pub fn status(&self) -> MassiveOutbreakSpawnerStatus {
        self.status.into()
    }

    pub fn has_base(&self) -> bool {
        self.base_table != 0 && self.base_table != 0xCBF29CE484222645
    }

    pub fn has_bonus(&self) -> bool {
        self.bonus_table != 0 && self.bonus_table != 0xCBF29CE484222645
    }
}
