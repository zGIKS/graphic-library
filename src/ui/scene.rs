#[derive(Clone, Copy, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: [f32; 4],
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32, color: [f32; 4]) -> Self {
        Self {
            x,
            y,
            width,
            height,
            color,
        }
    }

    // Geometry is generated on the GPU; this struct stays in pixel-space.
}

pub struct Scene {
    rects: Vec<Rect>,
    pub width: f32,
    pub height: f32,
    version: u64,
}

impl Scene {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            rects: Vec::with_capacity(1024),
            width,
            height,
            version: 0,
        }
    }

    pub fn clear(&mut self) {
        if !self.rects.is_empty() {
            self.rects.clear();
            self.version = self.version.wrapping_add(1);
        }
    }

    pub fn add_rect(&mut self, rect: Rect) {
        self.rects.push(rect);
        self.version = self.version.wrapping_add(1);
    }

    pub fn update_size(&mut self, width: f32, height: f32) {
        self.width = width.max(1.0);
        self.height = height.max(1.0);
    }

    pub fn rects(&self) -> &[Rect] {
        &self.rects
    }

    pub fn version(&self) -> u64 {
        self.version
    }
}
