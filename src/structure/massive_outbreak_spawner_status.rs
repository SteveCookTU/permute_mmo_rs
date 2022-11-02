#[derive(Copy, Clone, Default, Ord, PartialOrd, Eq, PartialEq)]
pub enum MassiveOutbreakSpawnerStatus {
    #[default]
    None,
    Unrevealed,
    Normal,
    Star,
    Aguav,
}

impl From<u8> for MassiveOutbreakSpawnerStatus {
    fn from(num: u8) -> Self {
        match num {
            1 => MassiveOutbreakSpawnerStatus::Unrevealed,
            2 => MassiveOutbreakSpawnerStatus::Normal,
            3 => MassiveOutbreakSpawnerStatus::Star,
            4 => MassiveOutbreakSpawnerStatus::Aguav,
            _ => MassiveOutbreakSpawnerStatus::None,
        }
    }
}
