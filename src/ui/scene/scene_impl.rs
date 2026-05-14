use super::rect::{Rect, ClipRect, clip_rect};
use super::text::Text;

pub struct Scene {
    rects: Vec<Rect>,
    texts: Vec<Text>,
    pub width: f32,
    pub height: f32,
    version: u64,
    clip_stack: Vec<ClipRect>,
    offset_stack: Vec<(f32, f32)>,
    offset: (f32, f32),
}

impl Scene {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            rects: Vec::with_capacity(1024),
            texts: Vec::with_capacity(64),
            width,
            height,
            version: 0,
            clip_stack: Vec::with_capacity(8),
            offset_stack: Vec::with_capacity(8),
            offset: (0.0, 0.0),
        }
    }

    pub fn clear(&mut self) {
        if !self.rects.is_empty() || !self.texts.is_empty() {
            self.rects.clear();
            self.texts.clear();
            self.version = self.version.wrapping_add(1);
        }
        self.clip_stack.clear();
        self.offset_stack.clear();
        self.offset = (0.0, 0.0);
    }

    pub fn add_rect(&mut self, rect: Rect) {
        let mut rect = Rect {
            x: rect.x + self.offset.0,
            y: rect.y + self.offset.1,
            ..rect
        };

        if let Some(clip) = self.current_clip() {
            let Some(clipped) = clip_rect(rect, clip) else {
                return;
            };
            rect = clipped;
        }

        self.rects.push(rect);
        self.version = self.version.wrapping_add(1);
    }

    pub fn add_text(&mut self, text: Text) {
        let mut text = Text {
            x: text.x + self.offset.0,
            y: text.y + self.offset.1,
            ..text
        };

        let text_rect = ClipRect::new(text.x, text.y, text.width, text.height);
        text.clip = match (text.clip, self.current_clip()) {
            (Some(a), Some(b)) => a.intersect(b),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (None, None) => None,
        }
        .and_then(|clip| clip.intersect(text_rect));

        if self.current_clip().is_some() && text.clip.is_none() {
            return;
        }

        self.texts.push(text);
        self.version = self.version.wrapping_add(1);
    }

    pub fn push_clip(&mut self, clip: ClipRect) {
        let clip = match self.current_clip() {
            Some(current) => current.intersect(clip),
            None => Some(clip),
        };

        if let Some(clip) = clip {
            self.clip_stack.push(clip);
        } else {
            self.clip_stack.push(ClipRect::new(0.0, 0.0, 0.0, 0.0));
        }
    }

    pub fn pop_clip(&mut self) {
        self.clip_stack.pop();
    }

    pub fn push_offset(&mut self, x: f32, y: f32) {
        self.offset_stack.push(self.offset);
        self.offset.0 += x;
        self.offset.1 += y;
    }

    pub fn pop_offset(&mut self) {
        if let Some(offset) = self.offset_stack.pop() {
            self.offset = offset;
        }
    }

    pub fn update_size(&mut self, width: f32, height: f32) {
        self.width = width.max(1.0);
        self.height = height.max(1.0);
    }

    pub fn rects(&self) -> &[Rect] {
        &self.rects
    }

    pub fn texts(&self) -> &[Text] {
        &self.texts
    }

    pub fn version(&self) -> u64 {
        self.version
    }

    fn current_clip(&self) -> Option<ClipRect> {
        self.clip_stack.last().copied()
    }
}
