use rogalik::prelude::*;

use crate::{
    globals::{BUBBLE_AGE, BUBBLE_SPEED, BUBBLE_Z, DIGITS_TEXT_SIZE, ICON_SIZE},
    ui::Span,
};

pub struct Bubble {
    pub origin: Vector2f,
    pub age: u32,
    pub color: Color,
    pub text: Option<String>,
    pub icon: Option<usize>,
}
impl Bubble {
    pub fn new(origin: Vector2f, color: Color, text: Option<String>, icon: Option<usize>) -> Self {
        Self {
            origin,
            color,
            text,
            icon,
            age: 0,
        }
    }
}

pub(crate) fn update_bubbles(bubbles: &mut Vec<Bubble>, context: &mut Context) {
    move_bubbles(bubbles);
    remove_old_bubbles(bubbles);
    draw_bubbles(bubbles, context);
}

fn remove_old_bubbles(bubbles: &mut Vec<Bubble>) {
    bubbles.retain(|a| a.age < BUBBLE_AGE);
}

fn move_bubbles(bubbles: &mut Vec<Bubble>) {
    for bubble in bubbles.iter_mut() {
        bubble.origin += Vector2f::new(0., BUBBLE_SPEED);
        bubble.age += 1;
    }
}

fn draw_bubbles(bubbles: &mut Vec<Bubble>, context: &mut Context) {
    for bubble in bubbles.iter() {
        let mut span = Span::new().with_font("digits");
        if let Some(text) = &bubble.text {
            span = span
                .with_text_borrowed(text)
                .with_text_size(DIGITS_TEXT_SIZE)
                .with_text_color(bubble.color)
                .with_spacer(2.);
        };
        if let Some(icon) = bubble.icon {
            span = span
                .with_sprite("icons_small", icon)
                .with_sprite_size(ICON_SIZE)
                .with_sprite_color(bubble.color);
        };
        span.draw(bubble.origin.round(), BUBBLE_Z, context);
    }
}
