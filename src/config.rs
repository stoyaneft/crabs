pub struct GameConfig {
    pub crab: CrabConfig,
    pub map: MapConfig,
}

pub struct CrabConfig {
    pub image: String,
    pub width: u16,
    pub height: u16,
}

pub struct MapConfig {
    pub image: String,
}
