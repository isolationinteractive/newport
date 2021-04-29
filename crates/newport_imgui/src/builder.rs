use crate::{
    Button, 
    ButtonResponse, 
    Context, 
    Id, 
    InputState, 
    Label, 
    Layout, 
    Painter, 
    Style
};

use crate::math::{ Vector2, Rect };

pub struct Builder<'a> {
    pub id:      Id,
    pub layout:  Layout,

    pub painter: Painter,
    pub(crate) context: &'a mut Context,
}

impl<'a> Builder<'a> {
    pub fn finish(self) {
        self.context.push_layer(self.painter)
    }

    pub fn input(&self) -> &InputState {
        &self.context.input
    }

    pub fn is_focused(&self, id: Id) -> bool {
        match &self.context.focused {
            Some(focused) => *focused == id,
            None => false
        }
    }

    pub fn is_hovered(&self, id: Id) -> bool {
        match &self.context.hovered {
            Some(hovered) => *hovered == id,
            None => false
        }
    }

    pub fn focus(&mut self, id: Id) -> bool {
        if self.context.focused.is_none() {
            self.context.focused = Some(id);
            return true;
        }
        false
    }

    pub fn force_focus(&mut self, id: Id) {
        self.context.focused = Some(id);
    }

    pub fn unfocus(&mut self, id: Id) -> bool{
        if self.is_focused(id) {
            self.context.focused = None;
            return true;
        }
        false
    }

    pub fn hover(&mut self, id: Id) {
        self.context.hovered = Some(id);
    }

    pub fn unhover(&mut self, id: Id) -> bool {
        if self.is_hovered(id) {
            self.context.hovered = None;
            return true;
        }
        false
    }

    #[must_use = "If a response is not being used then use a label"]
    pub fn button(&mut self, label: impl Into<String>) -> ButtonResponse {
        Button::new(label).build(self)
    }

    pub fn label(&mut self, label: impl Into<String>){
        Label::new(label).build(self)
    }

    pub fn layout(&mut self, layout: Layout, content: impl FnOnce(&mut Builder)) {
        let current = self.layout;
        self.layout = layout;
        content(self);
        self.layout = current;
    }

    pub fn available_rect(&self) -> Rect {
        self.layout.available_rect()
    }

    pub fn style(&self) -> Style {
        self.context.style()
    }

    pub fn set_style(&mut self, style: Style) {
        self.context.set_style(style);
    }

    pub fn content_bounds(&mut self, space_needed: Vector2) -> Rect {
        let style = self.style();

        let space_available = self.layout.space_left();
        let content_size = style.content_size(space_needed, space_available);

        let layout_rect = self.layout.push_size(style.spacing_size(content_size));
        
        Rect::from_pos_size(layout_rect.pos(), content_size)
    }

    pub fn add_spacing(&mut self, amount: f32) {
        self.layout.push_size(Vector2::new(amount, amount));
    }
}