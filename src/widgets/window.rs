use crate::{
    types::Rect,
    ui::WindowContext,
    Id, Ui,
};
use glam::Vec2;

#[derive(Debug, Clone)]
pub struct Window {
    id: Id,
    position: Vec2,
    size: Vec2,
    close_button: bool,
    enabled: bool,
    movable: bool,
    titlebar: bool,
    label: Option<String>,
}

impl Window {
    pub fn new(id: Id, position: Vec2, size: Vec2) -> Window {
        Window {
            id,
            position,
            size,
            close_button: false,
            enabled: true,
            movable: true,
            titlebar: true,
            label: None,
        }
    }

    pub fn label(self, label: &str) -> Window {
        Window {
            label: Some(label.to_string()),
            ..self
        }
    }

    pub fn movable(self, movable: bool) -> Window {
        Window { movable, ..self }
    }

    pub fn close_button(self, close_button: bool) -> Window {
        Window {
            close_button,
            ..self
        }
    }

    pub fn titlebar(self, titlebar: bool) -> Window {
        Window { titlebar, ..self }
    }

    pub fn enabled(self, enabled: bool) -> Window {
        Window { enabled, ..self }
    }

    pub fn ui<F: FnOnce(&mut Ui)>(self, ui: &mut Ui, f: F) -> bool {
        let token = self.begin(ui);
        f(ui);
        token.end(ui)
    }

    pub fn begin(self, ui: &mut Ui) -> WindowToken {
        let title_height = if self.titlebar {
            ui.style.title_height
        } else {
            0.
        };

        let context = ui.begin_window(
            self.id,
            None,
            self.position,
            self.size,
            title_height,
            self.movable,
        );

        // TODO: this will make each new window focused(appeared on the top) always
        // consider adding some configuration to be able to spawn background windows
        if context.window.was_active == false {
            ui.focus_window(self.id);
        }

        let mut context = ui.get_active_window_context();

        self.draw_window_frame(&mut context);
        if self.close_button && self.draw_close_button(&mut context) {
            context.close();
        }

        let clip_rect = context.window.content_rect();
        context.scroll_area();

        context.window.draw_commands.clip(clip_rect);

        WindowToken
    }

    fn draw_close_button(&self, context: &mut WindowContext) -> bool {
        let button_rect = Rect::new(
            context.window.position + Vec2::new(context.window.size.x() - 15.0, 0.0),
            Vec2::one() * 20.0
        );
        context.window.draw_commands.draw_label(
            "X",
            context.window.position + Vec2::new(context.window.size.x() - 10., 3.0),
            Some(context.global_style.title(context.focused)),
        );
        context.focused
            && button_rect.contains(context.input.mouse_position)
            && context.input.click_up
    }

    fn draw_window_frame(&self, context: &mut WindowContext) {
        let focused = context.focused;
        let style = context.global_style;
        let position = context.window.position;
        let size = context.window.size;

        context.window.draw_commands.draw_rect(
            Rect::new(position, size),
            style.window_border(focused),
            style.background(focused),
        );

        if self.titlebar {
            if let Some(label) = &self.label {
                context.window.draw_commands.draw_label(
                    &label,
                    position + Vec2::one() * style.margin,
                    context.global_style.title(focused),
                );
            }
            context.window.draw_commands.draw_line(
                position + Vec2::new(0.0, style.title_height),
                position + Vec2::new(size.x(), style.title_height),
                style.window_border(focused),
            );
        }
    }
}

#[must_use = "Must call `.end()` to finish Window"]
pub struct WindowToken;

impl WindowToken {
    pub fn end(self, ui: &mut Ui) -> bool {
        let context = ui.get_active_window_context();
        context.window.draw_commands.clip(None);

        let opened = context.window.want_close == false;

        ui.end_window();

        opened
    }
}

impl Ui {
    pub fn window<F: FnOnce(&mut Ui)>(
        &mut self,
        id: Id,
        position: Vec2,
        size: Vec2,
        f: F,
    ) -> bool {
        Window::new(id, position, size).ui(self, f)
    }
}
