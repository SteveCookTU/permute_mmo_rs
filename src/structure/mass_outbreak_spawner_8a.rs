use no_std_io::EndianRead;

#[derive(EndianRead, Default, Copy, Clone)]
pub struct MassOutbreakSpawner8a {
    pub display_species: u16,
    #[no_std_io(pad_before = 2)]
    pub display_form: u16,
    #[no_std_io(pad_before = 0x12)]
    pub area_hash: u64,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    #[no_std_io(pad_before = 8)]
    pub count_seed: u64,
    pub group_seed: u64,
    pub base_count: u8,
    #[no_std_io(pad_before = 3)]
    pub spawned_count: u32,
    pub padding: [u8; 4],
}

impl MassOutbreakSpawner8a {
    pub const SIZE: usize = 0x50;

    pub fn has_outbreak(&self) -> bool {
        self.area_hash != 0 && self.area_hash != 0xCBF29CE484222645
    }

    pub fn is_valid(&self) -> bool {
        self.display_species != 0
    }
}
