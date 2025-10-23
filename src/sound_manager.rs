use std::collections::HashMap;
use super::resource_manager::ResourceManager;
use macroquad::audio::{play_sound, play_sound_once, stop_sound, set_sound_volume, PlaySoundParams};

pub struct SoundManager<'a> {
    resource_manager: &'a ResourceManager,
    playing_state: HashMap<String, bool>,
}

impl<'a> SoundManager<'a> {
    pub fn new(resource_manager: &'a ResourceManager) -> Self {
        SoundManager {
            resource_manager,
            playing_state: HashMap::new(),
        }
    }

    pub fn play_once(&self, id: &str) {
        let sound = self.resource_manager.get_sound(id).unwrap();
        play_sound_once(sound);
    }

    pub fn start_playing(&mut self, id: &str, volume: f32) {
        if !self.is_playing(id) {
            let sound = self.resource_manager.get_sound(id).unwrap();
            play_sound(
                sound,
                PlaySoundParams {
                    looped: true,
                    volume,
                }
            );
            self.playing_state.insert(id.to_string(), true);
        }
    }

    pub fn stop_playing(&mut self, id: &str) {
        if self.is_playing(id) {
            let sound = self.resource_manager.get_sound(id).unwrap();
            stop_sound(sound);
            self.playing_state.insert(id.to_string(), false);
        }
    }

    pub fn set_volume(&self, id: &str, volume: f32) {
        if self.is_playing(id) {
            let sound = self.resource_manager.get_sound(id).unwrap();
            set_sound_volume(sound, volume);
        }
    }

    fn is_playing(&self, id:&str) -> bool {
        if let Some(playing) = self.playing_state.get(id) {
            *playing
        } else {
            false
        }
    }

}

