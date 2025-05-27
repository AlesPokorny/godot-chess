use std::fmt::Display;

use godot::builtin::{Vector2, Vector2i};

pub struct GodotPosition(u8, u8);

#[allow(dead_code)]
impl GodotPosition {
    pub const ZERO: Self = Self(0, 0);

    pub fn from_u8(x: u8, y: u8) -> Self {
        Self(x, y)
    }

    pub fn from_u32(x: u32, y: u32) -> Self {
        Self(x as u8, y as u8)
    }

    pub fn from_i32(x: i32, y: i32) -> Self {
        Self(x as u8, y as u8)
    }

    pub fn from_f32(x: f32, y: f32) -> Self {
        Self(x as u8, y as u8)
    }

    pub fn from_usize(x: usize, y: usize) -> Self {
        Self(x as u8, y as u8)
    }

    pub fn from_field_index(i: usize) -> Self {
        Self((i % 8) as u8, (i / 8) as u8)
    }

    pub fn from_ui_vector2(vec: Vector2, square_size: f32) -> Self {
        Self((vec.x / square_size) as u8, (vec.y / square_size) as u8)
    }

    pub fn to_chess_vector2(&self) -> Vector2 {
        Vector2::new(self.0 as f32, self.1 as f32)
    }

    pub fn to_chess_vector2i(&self) -> Vector2i {
        Vector2i::new(self.0 as i32, self.1 as i32)
    }

    pub fn to_ui_vector2(&self, square_size: f32) -> Vector2 {
        Vector2::new(self.0 as f32 * square_size, self.1 as f32 * square_size)
    }

    pub fn to_ui_vector2i(&self, square_size: f32) -> Vector2i {
        Vector2i::new(self.0 as i32 * square_size as i32, self.1 as i32 * square_size as i32)
    }

    pub fn to_field_index(&self) -> usize {
        (self.0 + self.1 * 8) as usize
    }
}

impl Display for GodotPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}
