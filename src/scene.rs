use crate::renderer::Vertex;

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
        Self { x, y, width, height, color }
    }

    pub fn to_vertices(&self, scale_x: f32, scale_y: f32) -> [Vertex; 4] {
        let x1 = (self.x / scale_x) * 2.0 - 1.0;
        let y1 = 1.0 - (self.y / scale_y) * 2.0;
        let x2 = ((self.x + self.width) / scale_x) * 2.0 - 1.0;
        let y2 = 1.0 - ((self.y + self.height) / scale_y) * 2.0;

        let [r, g, b, a] = self.color;

        [
            Vertex::new(x1, y1, r, g, b, a),
            Vertex::new(x2, y1, r, g, b, a),
            Vertex::new(x1, y2, r, g, b, a),
            Vertex::new(x2, y2, r, g, b, a),
        ]
    }
}

pub struct Scene {
    pub rects: Vec<Rect>,
    pub width: f32,
    pub height: f32,
}

impl Scene {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            rects: Vec::new(),
            width,
            height,
        }
    }

    pub fn add_rect(&mut self, rect: Rect) {
        self.rects.push(rect);
    }

    pub fn update_size(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
    }

    pub fn rects(&self) -> &[Rect] {
        &self.rects
    }

    pub fn rects_mut(&mut self) -> &mut Vec<Rect> {
        &mut self.rects
    }
}
