pub struct GameConfig {
    pub crab: CrabConfig,
    pub map: MapConfig,
    pub weapons: WeaponsConfig,
}

pub struct CrabConfig {
    pub image: String,
    pub image_firing: String,
    pub width: u16,
    pub height: u16,
}

pub struct MapConfig {
    pub image: String,
}

pub struct WeaponsConfig {
    pub image: String,
}
