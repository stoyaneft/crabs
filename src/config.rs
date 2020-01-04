pub struct GameConfig {
    pub players: Vec<PlayerConfig>,
    pub map: MapConfig,
    pub weapons: WeaponsConfig,
    pub players_count: u8,
}

pub struct CrabConfig {
    pub image: String,
    pub image_firing: String,
    pub width: u16,
    pub height: u16,
}

pub struct PlayerConfig {
    pub name: String,
    pub crabs_count: u8,
    pub crab: CrabConfig,
}

pub struct MapConfig {
    pub image: String,
}

pub struct WeaponsConfig {
    pub image: String,
}
