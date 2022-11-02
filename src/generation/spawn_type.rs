#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum SpawnType {
    Regular = 7,
    MMO = 7 + 12,
    Outbreak = 7 + 25,
}
