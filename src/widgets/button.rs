use crate::{Layout, Rect, Ui};
use glam::Vec2;

use std::borrow::Cow;

pub struct Button<'a> {
    position: Option<Vec2>,
    size: Option<Vec2>,
    label: Cow<'a, str>,
}

impl<'a> Button<'a> {
    pub fn new<S>(label: S) -> Button<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Button {
            position: None,
            size: None,
            label: label.into(),
        }
    }

    pub fn position<P: Into<Option<Vec2>>>(self, position: P) -> Self {
        let position = position.into();

        Button { position, ..self }
    }

    pub fn size(self, size: Vec2) -> Self {
        Button {
            size: Some(size),
            ..self
        }
    }

    pub fn ui(self, ui: &mut Ui) -> bool {
        let context = ui.get_active_window_context();

        let size = self.size.unwrap_or_else(|| {
            context.window.draw_commands.label_size(&self.label, None)
                + Vec2::new(2.0, 1.0) * context.global_style.margin_button
        });

        let pos = context
            .window
            .cursor
            .fit(size, self.position.map_or(Layout::Vertical, Layout::Free));
        let rect = Rect::new(pos, size);
        let hovered = rect.contains(context.input.mouse_position);

        context.window.draw_commands.draw_rect(
            rect,
            None,
            context.global_style.button_background(
                context.focused,
                hovered,
                hovered && context.input.is_mouse_down,
            ),
        );
        context.window.draw_commands.draw_label(
            &self.label,
            pos + Vec2::one() * context.global_style.margin_button,
            Some(context.global_style.text(context.focused)),
        );

        context.focused && hovered && context.input.click_up()
    }
}

impl Ui {
    pub fn button<P: Into<Option<Vec2>>>(&mut self, position: P, label: &str) -> bool {
        Button::new(label).position(position).ui(self)
    }
}
