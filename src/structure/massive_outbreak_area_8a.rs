use crate::structure::MassiveOutbreakSpawner8a;
use no_std_io::{Cursor, EndianRead, Error, ReadOutput, Reader, StreamContainer, StreamReader};
use std::ops::Index;

#[derive(Default, Copy, Clone, Debug)]
pub struct MassiveOutbreakArea8a {
    pub area_hash: u64,
    pub is_active: bool,
    spawners: [MassiveOutbreakSpawner8a; MassiveOutbreakArea8a::SPAWNER_COUNT],
}

impl MassiveOutbreakArea8a {
    pub const SIZE: usize = 0xB80;
    pub const SPAWNER_COUNT: usize = 20;

    pub fn is_valid(&self) -> bool {
        self.area_hash != 0 && self.area_hash != 0xCBF29CE484222645
    }
}

impl EndianRead for MassiveOutbreakArea8a {
    fn try_read_le(bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        let mut container = StreamContainer::new(bytes);
        let area_hash = container.default_read_stream_le::<u64>();
        let is_active = container.default_read_stream_le::<u8>() == 1;

        let mut spawners =
            [MassiveOutbreakSpawner8a::default(); MassiveOutbreakArea8a::SPAWNER_COUNT];
        for i in 0..MassiveOutbreakArea8a::SPAWNER_COUNT {
            let offset = 0x10 + (i * MassiveOutbreakSpawner8a::SIZE);
            spawners[i] = bytes.default_read_le(offset);
        }
        Ok(ReadOutput::new(
            Self {
                area_hash,
                is_active,
                spawners,
            },
            container.get_index(),
        ))
    }

    fn try_read_be(_bytes: &[u8]) -> Result<ReadOutput<Self>, Error> {
        unimplemented!()
    }
}

impl Index<usize> for MassiveOutbreakArea8a {
    type Output = MassiveOutbreakSpawner8a;

    fn index(&self, index: usize) -> &Self::Output {
        &self.spawners[index]
    }
}
