pub static CONFIG: GameConfig = GameConfig {
    screen: Screen {
        width: 1321.0,
        height: 960.0,
    },
    players_count: 2,
    players: [
        PlayerConfig {
            name: "Stoyan",
            crabs_count: 3,
            crab: CrabConfig {
                image: "/crab.png",
                image_firing: "/crab-firing.png",
                width: 48,
                height: 32,
            },
        },
        PlayerConfig {
            name: "PC",
            crabs_count: 3,
            crab: CrabConfig {
                image: "/crab2.png",
                image_firing: "/crab-firing2.png",
                width: 48,
                height: 32,
            },
        },
    ],
    map: MapConfig {
        image: "/map.png",
    },
    weapons: WeaponsConfig {
        image: "/weapons.png",
    },
    shots: ShotsConfig {
        pistol: ShotConfig {
            image: "/bullet.png",
            width: 15.0,
            height: 12.0,
            damage: 10.0,
            speed: 250.0,
            mass: 0.0,
        },
        bazooka: ShotConfig {
            image: "/bullet.png",
            width: 20.0,
            height: 10.0,
            damage: 25.0,
            speed: 250.0,
            mass: 500.0,
        },
        power: PowerConfig {
            min: 0.0,
            max: 2.0,
            time: 3.0,
        },
    },
    aim: ImageConfig {
        image: "/aim.png",
        width: 30,
        height: 30,
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

pub struct PowerConfig {
    pub min: f32,
    pub max: f32,
    pub time: f32,
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
    pub pistol: ShotConfig,
    pub bazooka: ShotConfig,
    pub power: PowerConfig,
}

pub struct ShotConfig {
    pub image: &'static str,
    pub speed: f32,
    pub damage: f32,
    pub mass: f32,
    pub width: f32,
    pub height: f32,
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
