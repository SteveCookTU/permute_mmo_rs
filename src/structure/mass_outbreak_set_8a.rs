use crate::structure::MassOutbreakSpawner8a;
use no_std_io::Reader;
use std::ops::Index;

#[derive(Copy, Clone, Default)]
pub struct MassOutbreakSet8a {
    spawners: [MassOutbreakSpawner8a; 5],
}

impl MassOutbreakSet8a {
    pub const SIZE: usize = 0x190;
    pub const AREA_COUNT: usize = 5;
}

impl From<&[u8]> for MassOutbreakSet8a {
    fn from(data: &[u8]) -> Self {
        let mut spawners = [MassOutbreakSpawner8a::default(); MassOutbreakSet8a::AREA_COUNT];
        debug_assert!(
            data.len() >= MassOutbreakSet8a::SIZE,
            "Data length is to small for MassOutbreakSet8a"
        );
        for i in 0..MassOutbreakSet8a::AREA_COUNT {
            let offset = i * MassOutbreakSpawner8a::SIZE;
            spawners[i] = data.default_read_le::<MassOutbreakSpawner8a>(offset);
        }
        Self { spawners }
    }
}

impl Index<usize> for MassOutbreakSet8a {
    type Output = MassOutbreakSpawner8a;

    fn index(&self, index: usize) -> &Self::Output {
        &self.spawners[index]
    }
}
