use rogalik::prelude::*;
use std::borrow::Cow;

use crate::globals::{BASE_TEXT_SIZE, PRIMARY_COLOR};

#[derive(Debug, Clone)]
pub enum SpanItem<'a> {
    Sprite(&'a str, usize),
    Text(Cow<'a, str>),
    Spacer(f32),
}

#[derive(Debug, Clone)]
pub struct Span<'a> {
    font: &'static str,
    text_color: Color,
    sprite_color: Color,
    text_size: f32,
    sprite_size: f32,
    items: Vec<SpanItem<'a>>,
}
impl<'a> Span<'a> {
    pub fn new() -> Self {
        Span {
            font: "default",
            text_size: BASE_TEXT_SIZE,
            sprite_size: BASE_TEXT_SIZE,
            sprite_color: Color(255, 255, 255, 255),
            text_color: PRIMARY_COLOR,
            items: Vec::new(),
        }
    }
    pub fn with_text_borrowed(mut self, text: &'a str) -> Self {
        self.items.push(SpanItem::Text(Cow::Borrowed(text)));
        self
    }
    pub fn with_text_owned(mut self, text: String) -> Self {
        self.items.push(SpanItem::Text(Cow::Owned(text)));
        self
    }
    pub fn with_sprite(mut self, atlas: &'a str, index: usize) -> Self {
        self.items.push(SpanItem::Sprite(atlas, index));
        self
    }
    pub fn with_spacer(mut self, width: f32) -> Self {
        self.items.push(SpanItem::Spacer(width));
        self
    }
    pub fn with_text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }
    pub fn with_sprite_color(mut self, color: Color) -> Self {
        self.sprite_color = color;
        self
    }
    pub fn with_text_size(mut self, size: f32) -> Self {
        self.text_size = size;
        self
    }
    pub fn with_sprite_size(mut self, size: f32) -> Self {
        self.sprite_size = size;
        self
    }
    pub fn with_font(mut self, font: &'static str) -> Self {
        self.font = font;
        self
    }
    pub fn width(&self, context: &Context) -> f32 {
        let mut width = 0.;
        for item in self.items.iter() {
            match item {
                SpanItem::Text(text) => {
                    width += context
                        .graphics
                        .text_dimensions(self.font, text, self.text_size as f32)
                        .x
                }
                &SpanItem::Sprite(_, _) => width += self.sprite_size as f32,
                SpanItem::Spacer(w) => width += w,
            }
        }
        width
    }
    pub fn height(&self) -> f32 {
        self.text_size.max(self.sprite_size)
    }

    pub fn draw(&self, origin: Vector2f, z: i32, context: &mut Context) {
        let mut offset = 0.;
        let middle = Vector2f::new(origin.x, origin.y - 0.5 * self.height());

        for item in self.items.iter() {
            match item {
                SpanItem::Text(text) => {
                    let _ = context.graphics.draw_text(
                        &self.font,
                        text,
                        (middle + Vector2f::new(offset, -0.5 * self.text_size)).round(),
                        z,
                        self.text_size,
                        SpriteParams {
                            color: self.text_color,
                            ..Default::default()
                        },
                    );
                    offset += context
                        .graphics
                        .text_dimensions(&self.font, text, self.text_size)
                        .x;
                }
                &SpanItem::Sprite(atlas, index) => {
                    let _ = context.graphics.draw_atlas_sprite(
                        atlas,
                        index,
                        (middle + Vector2f::new(offset, -0.5 * self.sprite_size)).round(),
                        z,
                        Vector2f::new(self.sprite_size, self.sprite_size),
                        SpriteParams {
                            color: self.sprite_color,
                            ..Default::default()
                        },
                    );
                    offset += self.sprite_size;
                }
                SpanItem::Spacer(w) => offset += w,
            }
        }
    }
}
