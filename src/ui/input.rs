use glam::Vec2;

pub use crate::input_handler::KeyCode;

#[derive(Clone, Debug)]
pub enum Key {
    Char(char),
    KeyCode(KeyCode),
}

#[derive(Clone, Debug)]
pub struct InputCharacter {
    pub key: Key,
    pub modifier_shift: bool,
    pub modifier_ctrl: bool,
}

#[derive(Default, Clone)]
pub struct Input {
    pub(crate) mouse_position: Vec2,
    pub(crate) is_mouse_down: bool,
    pub(crate) click_down: bool,
    pub(crate) click_up: bool,
    pub(crate) mouse_wheel: Vec2,
    pub(crate) input_buffer: Vec<InputCharacter>,
    pub(crate) cursor_grabbed: bool,
    // TODO: its a hack to prevent button click behind modal
    pub(crate) modal_active: bool,
}

impl Input {
    pub fn is_mouse_down(&self) -> bool {
        self.is_mouse_down && self.cursor_grabbed == false && self.modal_active == false
    }

    pub fn click_down(&self) -> bool {
        self.click_down && self.cursor_grabbed == false && self.modal_active == false
    }

    pub fn click_up(&self) -> bool {
        self.click_up && self.cursor_grabbed == false && self.modal_active == false
    }

    pub fn reset(&mut self) {
        self.click_down = false;
        self.click_up = false;
        self.modal_active = false;
        self.mouse_wheel = Vec2::zero();
        self.input_buffer = vec![];
    }
}
