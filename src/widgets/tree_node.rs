use crate::{
    types::Rect,
    Id, Layout, Ui,
};
use glam::Vec2;

use std::borrow::Cow;

pub struct TreeNode<'a> {
    id: Id,
    label: Cow<'a, str>,
    init_unfolded: bool,
}

impl<'a> TreeNode<'a> {
    pub fn new<S>(id: Id, label: S) -> TreeNode<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        TreeNode {
            id,
            label: label.into(),
            init_unfolded: false,
        }
    }

    pub fn init_unfolded(mut self) -> TreeNode<'a> {
	self.init_unfolded = true;
	self
    }

    pub fn ui<F: FnOnce(&mut Ui)>(self, ui: &mut Ui, f: F) -> bool {
        if let Some(token) = self.begin(ui) {
            f(ui);
            token.end(ui)
        } else {
            false
        }
    }

    pub fn begin(self, ui: &mut Ui) -> Option<TreeNodeToken> {
        let context = ui.get_active_window_context();

        let size = Vec2::new(300., 14.);

        let color = context.global_style.text(context.focused);
        let pos = context.window.cursor.fit(size, Layout::Vertical);

        let hovered = Rect::new(pos, size).contains(context.input.mouse_position);

        let clicked = context.focused && hovered && context.input.click_down();

        let opened = context
            .storage_u32
            .entry(self.id)
            .or_insert(if self.init_unfolded { 1 } else { 0 });

        if clicked {
            *opened ^= 1;
        }

        context
            .window
            .draw_commands
            .draw_label(if *opened == 0 { "+" } else { "-" }, pos, color);
        context
            .window
            .draw_commands
            .draw_label(&*self.label, pos + Vec2::new(10., 0.), color);

        if *opened == 1 {
            context.window.cursor.ident += 5.;

            Some(TreeNodeToken {
                clicked,
            })
        } else {
            None
        }
    }
}

#[must_use = "Must call `.end()` to finish TreeNode"]
pub struct TreeNodeToken {
    clicked: bool,
}

impl TreeNodeToken {
    pub fn end(self, ui: &mut Ui) -> bool {
        let context = ui.get_active_window_context();
        context.window.cursor.ident -= 5.;

        self.clicked
    }
}

impl Ui {
    pub fn tree_node<F: FnOnce(&mut Ui)>(&mut self, id: Id, label: &str, f: F) -> bool {
        TreeNode::new(id, label).ui(self, f)
    }
}
