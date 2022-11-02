use crate::structure::MassiveOutbreakArea8a;
use no_std_io::Reader;
use std::ops::Index;

#[derive(Default, Copy, Clone, Debug)]
pub struct MassiveOutbreakSet8a {
    areas: [MassiveOutbreakArea8a; 5],
}

impl MassiveOutbreakSet8a {
    pub const SIZE: usize = 0x3980;
    pub const AREA_COUNT: usize = 5;
}

impl From<&[u8]> for MassiveOutbreakSet8a {
    fn from(data: &[u8]) -> Self {
        let mut areas = [MassiveOutbreakArea8a::default(); MassiveOutbreakSet8a::AREA_COUNT];
        for i in 0..MassiveOutbreakSet8a::AREA_COUNT {
            let offset = i * MassiveOutbreakArea8a::SIZE;
            areas[i] = data.default_read_le(offset);
        }
        Self { areas }
    }
}

impl Index<usize> for MassiveOutbreakSet8a {
    type Output = MassiveOutbreakArea8a;

    fn index(&self, index: usize) -> &Self::Output {
        &self.areas[index]
    }
}
