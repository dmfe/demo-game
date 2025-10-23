use macroquad::prelude::*;
use macroquad::experimental::animation::AnimatedSprite;
use super::resource_manager::ResourceManager;

pub struct GameObject {
    pub width: f32,
    pub height: f32,
    pub speed: f32,
    pub x: f32,
    pub y: f32,
    pub collided: bool,
    pub texture_id: String,
    pub sprite: AnimatedSprite,
    pub animation_num: usize,
}

impl GameObject {
    pub fn collides_with(&self, other: &Self) -> bool {
        self.rect().overlaps(&other.rect())
    }

    pub fn rect(&self) -> Rect {
        Rect {
            x: self.x - self.width / 2.0,
            y: self.y - self.height / 2.0,
            w: self.width,
            h: self.height
        }
    }

    pub fn draw(&mut self, resource_manager: &ResourceManager) {
        self.sprite.set_animation(self.animation_num);
        self.sprite.update();
        let frame = self.sprite.frame();
        let texture = resource_manager.get_texture(&self.texture_id).unwrap();
        draw_texture_ex(
            texture,
            self.x - self.width / 2.0,
            self.y - self.width / 2.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.width, self.height)),
                source: Some(frame.source_rect),
                ..Default::default()
            }
        );
    }

    pub fn set_animation_num(&mut self, num: usize) {
        self.animation_num = num;
    }

}

