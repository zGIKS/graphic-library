use super::rect::ClipRect;

#[derive(Clone, Debug, PartialEq)]
pub struct Text {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub size: f32,
    pub line_height: f32,
    pub color: [u8; 4],
    pub content: String,
    pub clip: Option<ClipRect>,
}

impl Text {
    pub fn new(
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        size: f32,
        content: impl Into<String>,
        color: [u8; 4],
    ) -> Self {
        Self {
            x,
            y,
            width,
            height,
            size,
            line_height: size * 1.25,
            color,
            content: content.into(),
            clip: None,
        }
    }
}