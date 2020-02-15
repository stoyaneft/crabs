pub static CONFIG: GameConfig = GameConfig {
    screen: Screen {
        // width: 1280.0,
        width: 500.0,
        // height: 720.0,
        height: 300.0,
    },
    players_count: 2,
    players: [
        PlayerConfig {
            name: "Stoyan",
            crabs_count: 1,
            crab: CrabConfig {
                image: "/crab.png",
                image_firing: "/crab-firing.png",
                width: 48,
                height: 32,
            },
        },
        PlayerConfig {
            name: "PC",
            crabs_count: 1,
            crab: CrabConfig {
                image: "/crab2.png",
                image_firing: "/crab-firing2.png",
                width: 48,
                height: 32,
            },
        },
    ],
    map: MapConfig {
        image: "/large-hill.png",
    },
    weapons: WeaponsConfig {
        image: "/weapons.png",
    },
    shots: ShotsConfig {
        pistol: ImageConfig {
            image: "/bullet.png",
            width: 15,
            height: 12,
        },
        bazooka: ImageConfig {
            image: "/bullet.png",
            width: 20,
            height: 10,
        },
    },
    aim: ImageConfig {
        image: "/aim.png",
        width: 20,
        height: 20,
    },
    arrow: ImageConfig {
        image: "/arrow.png",
        width: 20,
        height: 25,
    },
};

pub struct GameConfig {
    pub screen: Screen,
    pub players: [PlayerConfig; 2],
    pub map: MapConfig,
    pub weapons: WeaponsConfig,
    pub players_count: u8,
    pub shots: ShotsConfig,
    pub aim: ImageConfig,
    pub arrow: ImageConfig,
}

pub struct CrabConfig {
    pub image: &'static str,
    pub image_firing: &'static str,
    pub width: u16,
    pub height: u16,
}

pub struct PlayerConfig {
    pub name: &'static str,
    pub crabs_count: u8,
    pub crab: CrabConfig,
}

pub struct MapConfig {
    pub image: &'static str,
}

pub struct WeaponsConfig {
    pub image: &'static str,
}

pub struct ShotsConfig {
    pub pistol: ImageConfig,
    pub bazooka: ImageConfig,
}

pub struct ImageConfig {
    pub image: &'static str,
    pub width: u16,
    pub height: u16,
}

pub struct Screen {
    pub width: f32,
    pub height: f32,
}
