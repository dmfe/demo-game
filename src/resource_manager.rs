use std::collections::HashMap;
use macroquad::prelude::*;

pub mod constants {
    pub const PLAYER_TEX_ID: &str = "player_texture";
    pub const BULLET_TEX_ID: &str = "bullet_texture";
    pub const EXPLOSION_TEX_ID: &str = "explosion_texture";
    pub const ENEMY_SMALL_TEX_ID: &str = "enemy_small_texture";
    pub const ENEMY_MEDIUM_TEX_ID: &str = "enemy_medium_texture";
    pub const ENEMY_BIG_TEX_ID: &str = "enemy_big_texture";
    pub const ENEMY_TEXTURES: &[&str] = &[ENEMY_SMALL_TEX_ID, ENEMY_MEDIUM_TEX_ID, ENEMY_BIG_TEX_ID];
}

pub mod animations {
    use macroquad::experimental::animation::{AnimatedSprite, Animation};

    pub fn player_animation() -> AnimatedSprite {
        AnimatedSprite::new(
                16,
                24,
                &[
                    Animation {
                        name: "idle".to_string(),
                        row: 0,
                        frames: 2,
                        fps: 12,
                    },
                    Animation {
                        name: "slight-left".to_string(),
                        row: 1,
                        frames: 2,
                        fps: 12,
                    },
                    Animation {
                        name: "left".to_string(),
                        row: 2,
                        frames: 2,
                        fps: 12,
                    },
                    Animation {
                        name: "slight-right".to_string(),
                        row: 3,
                        frames: 2,
                        fps: 12,
                    },
                    Animation {
                        name: "right".to_string(),
                        row: 4,
                        frames: 2,
                        fps: 12,
                    }
                ],
                true
            )
    }

    pub fn bullet_animation() -> AnimatedSprite {
        AnimatedSprite::new(
                16,
                16,
                &[
                    Animation {
                        name: "bullet".to_string(),
                        row: 0,
                        frames: 2,
                        fps: 12
                    },
                    Animation {
                        name: "bolt".to_string(),
                        row: 1,
                        frames: 2,
                        fps: 12
                    },
                ],
                true
            )
    }

    pub fn enemy_small_animation() -> AnimatedSprite {
        AnimatedSprite::new(
                17,
                16,
                &[
                    Animation {
                        name: "enemy_small".to_string(),
                        row: 0,
                        frames: 2,
                        fps: 12
                    }
                ],
                true
            )
    }

    pub fn enemy_medium_animation() -> AnimatedSprite {
        AnimatedSprite::new(
                32,
                16,
                &[
                    Animation {
                        name: "enemy_medium".to_string(),
                        row: 0,
                        frames: 2,
                        fps: 12
                    }
                ],
                true
            )
    }

    pub fn enemy_big_animation() -> AnimatedSprite {
        AnimatedSprite::new(
                32,
                32,
                &[
                    Animation {
                        name: "enemy_big".to_string(),
                        row: 0,
                        frames: 2,
                        fps: 12
                    }
                ],
                true
            )
    }

}

pub struct ResourceManager {
    resources: HashMap<String, Texture2D>
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            resources: HashMap::new(),
        }
    }

    pub async fn load_resources(&mut self) {
        set_pc_assets_folder("assets");
        let player_texture: Texture2D = load_texture("ship.png")
            .await
            .expect("Couldn't load texture file.");
        player_texture.set_filter(FilterMode::Nearest);
        let bullet_texture: Texture2D = load_texture("laser-bolts.png")
            .await
            .expect("Couldn't load texture file.");
        bullet_texture.set_filter(FilterMode::Nearest);
        let explosion_texture: Texture2D = load_texture("explosion.png")
            .await
            .expect("Couldn't load texture file.");
        explosion_texture.set_filter(FilterMode::Nearest);
        let enemy_small_texture: Texture2D = load_texture("enemy-small.png")
            .await
            .expect("Couldn't load texture file.");
        enemy_small_texture.set_filter(FilterMode::Nearest);
        let enemy_medium_texture: Texture2D = load_texture("enemy-medium.png")
            .await
            .expect("Couldn't load texture file.");
        enemy_medium_texture.set_filter(FilterMode::Nearest);
        let enemy_big_texture: Texture2D = load_texture("enemy-big.png")
            .await
            .expect("Couldn't load texture file.");
        enemy_big_texture.set_filter(FilterMode::Nearest);
        build_textures_atlas();

        self.resources.insert(constants::PLAYER_TEX_ID.to_string(), player_texture);
        self.resources.insert(constants::BULLET_TEX_ID.to_string(), bullet_texture);
        self.resources.insert(constants::EXPLOSION_TEX_ID.to_string(), explosion_texture);
        self.resources.insert(constants::ENEMY_SMALL_TEX_ID.to_string(), enemy_small_texture);
        self.resources.insert(constants::ENEMY_MEDIUM_TEX_ID.to_string(), enemy_medium_texture);
        self.resources.insert(constants::ENEMY_BIG_TEX_ID.to_string(), enemy_big_texture);
    }

    pub fn get_resource(&self, id: &str) -> Option<&Texture2D> {
        self.resources.get(id)
    }

}
