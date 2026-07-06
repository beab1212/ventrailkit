//! Geometry helpers for vent-field bounding boxes and depth intervals.

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BathyBox {
    pub min_x: f64,
    pub min_y: f64,
    pub max_x: f64,
    pub max_y: f64,
}

impl BathyBox {
    pub fn new(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> Self {
        Self { min_x: min_x.min(max_x), min_y: min_y.min(max_y), max_x: min_x.max(max_x), max_y: min_y.max(max_y) }
    }
    pub fn width(&self) -> f64 { self.max_x - self.min_x }
    pub fn height(&self) -> f64 { self.max_y - self.min_y }
    pub fn area(&self) -> f64 { self.width().abs() * self.height().abs() }
    pub fn center(&self) -> (f64, f64) { ((self.min_x + self.max_x) * 0.5, (self.min_y + self.max_y) * 0.5) }
    pub fn intersects(&self, other: &Self) -> bool {
        self.min_x <= other.max_x && self.max_x >= other.min_x && self.min_y <= other.max_y && self.max_y >= other.min_y
    }
    pub fn contains_point(&self, x: f64, y: f64) -> bool {
        x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y
    }
    pub fn expand(&self, margin: f64) -> Self {
        Self::new(self.min_x - margin, self.min_y - margin, self.max_x + margin, self.max_y + margin)
    }
}
