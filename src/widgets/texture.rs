use crate::{Layout, Rect, Ui};
use glam::Vec2;

pub struct Texture {
    position: Option<Vec2>,
    w: f32,
    h: f32,
    texture: u32,
}

impl Texture {
    pub fn new(texture: u32) -> Texture {
        Texture {
            position: None,
            w: 100.,
            h: 100.,
            texture,
        }
    }

    pub fn size(self, w: f32, h: f32) -> Self {
        Texture { w, h, ..self }
    }

    pub fn position<P: Into<Option<Vec2>>>(self, position: P) -> Self {
        let position = position.into();

        Texture { position, ..self }
    }

    pub fn ui(self, ui: &mut Ui) -> bool {
        let context = ui.get_active_window_context();

        let size = Vec2::new(self.w, self.h);

        let pos = context
            .window
            .cursor
            .fit(size, self.position.map_or(Layout::Vertical, Layout::Free));
        context
            .window
            .draw_commands
            .draw_raw_texture(Rect::new(pos, Vec2::new(self.w, self.h)), self.texture);

        let hovered = Rect::new(pos, size).contains(context.input.mouse_position);

        context.focused && hovered && context.input.click_up
    }
}

impl Ui {
    pub fn texture(&mut self, texture: u32, w: f32, h: f32) -> bool {
        Texture::new(texture).size(w, h).ui(self)
    }
}
