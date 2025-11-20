use std::collections::HashMap;
use macroquad::prelude::*;
use macroquad::audio::{load_sound, Sound};
use macroquad::experimental::collections::storage;
use macroquad::experimental::coroutines::{start_coroutine, Coroutine};

pub mod constants {
    pub const PLAYER_TEX_ID: &str = "player_texture";
    pub const BULLET_TEX_ID: &str = "bullet_texture";
    pub const EXPLOSION_TEX_ID: &str = "explosion_texture";
    pub const ENEMY_SMALL_TEX_ID: &str = "enemy_small_texture";
    pub const ENEMY_MEDIUM_TEX_ID: &str = "enemy_medium_texture";
    pub const ENEMY_BIG_TEX_ID: &str = "enemy_big_texture";
    pub const ENEMY_TEXTURES: &[&str] = &[ENEMY_SMALL_TEX_ID, ENEMY_MEDIUM_TEX_ID, ENEMY_BIG_TEX_ID];

    pub const WINDOW_BACKGROUND: &str = "window_background";
    pub const BUTTON_BACKGROUND: &str = "button_background";
    pub const BUTTON_PRESSED_BACKGROUND: &str = "button_pressed_background";

    pub const THEME_MUSIC: &str = "theme_music";
    pub const EXPLOSION_SOUND: &str = "explosion_sound";
    pub const LASER_SOUND: &str = "laser_sound";

    pub const FONT: &str = "font";
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
    textures: HashMap<String, Texture2D>,
    images: HashMap<String, Image>,
    sounds: HashMap<String, Sound>,
    fonts: HashMap<String, Vec<u8>>,
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            textures: HashMap::new(),
            images: HashMap::new(),
            sounds: HashMap::new(),
            fonts: HashMap::new(),
        }
    }

    pub async fn create_resource_manager() -> Result<(), macroquad::Error> {
        let resource_loading: Coroutine<Result<(), macroquad::Error>> = start_coroutine(async move {
            let mut resource_manager = ResourceManager::new();
            resource_manager.load_resources().await?;
            storage::store(resource_manager);
            Ok(())
        });

        while !resource_loading.is_done() {
            clear_background(BLACK);
            let text = format!(
                "Loading resources {}",
                ".".repeat(((get_time() * 2.0) as usize) % 4)
            );
            draw_text(
                &text,
                screen_width() / 2.0 - 160.0,
                screen_height() / 2.0,
                40.0,
                WHITE,
            );
            next_frame().await;
        }

        resource_loading.retrieve().unwrap()?;
        Ok(())
    }

    pub fn get_texture(&self, id: &str) -> Option<&Texture2D> {
        self.textures.get(id)
    }

    pub fn get_image(&self, id: &str) -> Option<&Image> {
        self.images.get(id)
    }

    pub fn get_sound(&self, id: &str) -> Option<&Sound> {
        self.sounds.get(id)
    }

    pub fn get_font(&self, id: &str) -> Option<&Vec<u8>> {
        self.fonts.get(id)
    }

    async fn load_resources(&mut self) -> Result<(), macroquad::Error> {
        set_pc_assets_folder("assets");
        self.load_textures().await?;
        self.load_images().await?;
        self.load_sounds().await?;
        self.load_fonts().await?;

        Ok(())
    }

    async fn load_textures(&mut self) -> Result<(), macroquad::Error> {
        let player_texture: Texture2D = load_texture("ship.png")
            .await?;
        player_texture.set_filter(FilterMode::Nearest);
        let bullet_texture: Texture2D = load_texture("laser-bolts.png")
            .await?;
        bullet_texture.set_filter(FilterMode::Nearest);
        let explosion_texture: Texture2D = load_texture("explosion.png")
            .await?;
        explosion_texture.set_filter(FilterMode::Nearest);
        let enemy_small_texture: Texture2D = load_texture("enemy-small.png")
            .await?;
        enemy_small_texture.set_filter(FilterMode::Nearest);
        let enemy_medium_texture: Texture2D = load_texture("enemy-medium.png")
            .await?;
        enemy_medium_texture.set_filter(FilterMode::Nearest);
        let enemy_big_texture: Texture2D = load_texture("enemy-big.png")
            .await?;
        enemy_big_texture.set_filter(FilterMode::Nearest);
        build_textures_atlas();

        self.textures.insert(constants::PLAYER_TEX_ID.to_string(), player_texture);
        self.textures.insert(constants::BULLET_TEX_ID.to_string(), bullet_texture);
        self.textures.insert(constants::EXPLOSION_TEX_ID.to_string(), explosion_texture);
        self.textures.insert(constants::ENEMY_SMALL_TEX_ID.to_string(), enemy_small_texture);
        self.textures.insert(constants::ENEMY_MEDIUM_TEX_ID.to_string(), enemy_medium_texture);
        self.textures.insert(constants::ENEMY_BIG_TEX_ID.to_string(), enemy_big_texture);

        Ok(())
    }

    async fn load_images(&mut self) -> Result<(), macroquad::Error> {
        let window_background = load_image("window_background.png")
            .await?;
        let button_background = load_image("button_background.png")
            .await?;
        let button_pressed_background = load_image("button_clicked_background.png")
            .await?;

        self.images.insert(constants::WINDOW_BACKGROUND.to_string(), window_background);
        self.images.insert(constants::BUTTON_BACKGROUND.to_string(), button_background);
        self.images.insert(constants::BUTTON_PRESSED_BACKGROUND.to_string(), button_pressed_background);

        Ok(())
    }

    async fn load_sounds(&mut self) -> Result<(), macroquad::Error> {
        let theme_music = load_sound("8bit-spaceshooter.ogg")
            .await?;
        let explosion_sound = load_sound("explosion.wav")
            .await?;
        let laser_sound = load_sound("laser.wav")
            .await?;

        self.sounds.insert(constants::THEME_MUSIC.to_string(), theme_music);
        self.sounds.insert(constants::EXPLOSION_SOUND.to_string(), explosion_sound);
        self.sounds.insert(constants::LASER_SOUND.to_string(), laser_sound);

        Ok(())
    }

    async fn load_fonts(&mut self) -> Result<(), macroquad::Error> {
        let font = load_file("atari_games.ttf")
            .await?;
        self.fonts.insert(constants::FONT.to_string(), font);

        Ok(())
    }

}

