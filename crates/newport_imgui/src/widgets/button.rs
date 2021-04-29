use crate::{Builder, ButtonResponse, Id, ToId, button_control};
use crate::math::Rect;

pub struct Button {
    id: Id,
    label: String,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        let label = label.into();

        Self {
            id:    Id::from(&label),
            label: label,
        }
    }

    pub fn id(mut self, id: impl ToId) -> Self {
        self.id = id.to_id();
        self
    }
}

impl Button {
    #[must_use = "If a response is not being used then use a label"]
    pub fn build(self, builder: &mut Builder) -> ButtonResponse {
        let style = builder.style();

        let label_rect = style.string_rect(&self.label, style.label_size, None).size();
        let bounds = builder.content_bounds(label_rect);
        
        let response = button_control(self.id, bounds, builder);
        
        let is_focused = builder.is_focused(self.id);
        let is_hovered = builder.is_hovered(self.id);
        
        let (background_color, foreground_color) = {
            let background_color = if is_focused {
                style.focused_background
            } else if is_hovered {
                style.hovered_background
            } else {
                style.unhovered_background
            };

            let foreground_color = if is_focused {
                style.focused_foreground
            } else if is_hovered {
                style.hovered_foreground
            } else {
                style.unhovered_foreground
            };

            (background_color, foreground_color)
        };

        builder.painter.rect(bounds).color(background_color);
        
        let at = Rect::from_pos_size(bounds.pos(), label_rect).top_left();
        builder.painter
            .text(self.label, at, &style.font, style.label_size, builder.input().dpi)
            .color(foreground_color)
            .scissor(bounds);

        response
    }
}