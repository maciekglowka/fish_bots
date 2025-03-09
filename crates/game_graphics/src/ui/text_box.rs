use rogalik::prelude::*;
use std::borrow::Cow;

use crate::globals::{BASE_TEXT_SIZE, PRIMARY_COLOR};

pub struct TextBox<'a> {
    text_color: Color,
    text_size: f32,
    text: Cow<'a, str>,
}
impl<'a> TextBox<'a> {
    pub fn borrowed(text: &'a str) -> Self {
        Self {
            text_size: BASE_TEXT_SIZE,
            text_color: PRIMARY_COLOR,
            text: Cow::Borrowed(text),
        }
    }
    pub fn owned(text: String) -> Self {
        Self {
            text_size: BASE_TEXT_SIZE,
            text_color: PRIMARY_COLOR,
            text: Cow::Owned(text),
        }
    }
    pub fn with_text_color(mut self, color: Color) -> Self {
        self.text_color = color;
        self
    }
    pub fn with_text_size(mut self, size: f32) -> Self {
        self.text_size = size;
        self
    }
    pub fn draw(&self, origin: Vector2f, width: f32, z: i32, context: &mut Context) -> f32 {
        let paragraphs = self.text.split('\n');
        let mut v_offset = 0.;
        let line_height = 1.1 * self.text_size;
        let space = context
            .graphics
            .text_dimensions("default", " ", self.text_size)
            .x;

        for paragraph in paragraphs {
            let mut line_width = 0.;
            let words = paragraph.split(" ");
            for word in words {
                let w = context
                    .graphics
                    .text_dimensions("default", word, self.text_size as f32)
                    .x;
                if line_width + w > width {
                    line_width = 0.;
                    v_offset += line_height;
                }

                let _ = context.graphics.draw_text(
                    "default",
                    word,
                    origin + Vector2f::new(line_width, -(self.text_size as f32) - v_offset),
                    z,
                    self.text_size as f32,
                    SpriteParams {
                        color: self.text_color,
                        ..Default::default()
                    },
                );
                line_width += w + space;
            }
            v_offset += line_height;
        }
        v_offset
    }
}
