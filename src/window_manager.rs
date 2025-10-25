use std::collections::HashMap;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, Skin, Ui, Id};
use super::resource_manager::{self as resource_manager, ResourceManager};

pub struct WindowManager<'a> {
    resource_manager: &'a ResourceManager,
    windows: HashMap<String, Id>,
}

impl<'a> WindowManager<'a> {
    pub fn new(resource_manager: &'a ResourceManager) -> Self {
        WindowManager {
            resource_manager,
            windows: HashMap::new(),
        }
    }

    pub fn configure_ui_skin(&self) {
        let window_background = self.resource_manager
            .get_image(resource_manager::constants::WINDOW_BACKGROUND).unwrap();
        let button_background = self.resource_manager
            .get_image(resource_manager::constants::BUTTON_BACKGROUND).unwrap();
        let button_pressed_background = self.resource_manager
            .get_image(resource_manager::constants::BUTTON_PRESSED_BACKGROUND).unwrap();
        let font = self.resource_manager
            .get_font(resource_manager::constants::FONT).unwrap();

        let window_style = root_ui()
            .style_builder()
            .background(window_background.clone())
            .background_margin(RectOffset::new(32.0, 76.0, 44.0, 20.0))
            .margin(RectOffset::new(0.0, -40.0, 0.0, 0.0))
            .build();
        let button_style = root_ui()
            .style_builder()
            .background(button_background.clone())
            .background_clicked(button_pressed_background.clone())
            .background_margin(RectOffset::new(16.0, 16.0, 16.0, 16.0))
            .margin(RectOffset::new(16.0, 0.0, -8.0, -8.0))
            .font(font)
            .unwrap()
            .text_color(WHITE)
            .font_size(64)
            .build();
        let label_style = root_ui()
            .style_builder()
            .font(font)
            .unwrap()
            .text_color(WHITE)
            .font_size(28)
            .build();
        let ui_skin = Skin {
            window_style,
            button_style,
            label_style,
            ..root_ui().default_skin()
        };
        root_ui().push_skin(&ui_skin);
    }

    pub fn window<F: FnOnce(&mut Ui)>(
        &mut self,
        name: &str,
        positon: Vec2,
        size: Vec2,
        f: F
    ) -> bool {
        let id = hash!();
        let result = root_ui().window(
            id,
            positon,
            size,
            f
        );
        if result {
            self.windows.insert(name.to_string(), id);
        }
        result
    }

    pub fn move_window(&self, id: Id, position: Vec2) {
        root_ui().move_window(id, position);
    }

    pub fn get_window_id(&self, name: &str) -> Option<Id> {
        self.windows.get(name).copied()
    }

}

