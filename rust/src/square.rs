use std::fmt::Display;

use godot::builtin::Vector2;
use rustier_chess::types::square::Square;

use crate::chess_pieces::GodotPieceColor;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct GodotSquare(Square);

#[allow(unused)]
impl GodotSquare {
    pub fn from_u8(x: u8, y: u8, player_color: &GodotPieceColor) -> Self {
        if player_color == &GodotPieceColor::White {
            return Self(Square::new(x + (7 - y) * 8));
        }
        Self(Square::new(y * 8 + 7 - x))
    }

    pub fn from_u32(x: u32, y: u32, player_color: &GodotPieceColor) -> Self {
        Self::from_u8(x as u8, y as u8, player_color)
    }

    pub fn from_i16(x: i16, y: i16, player_color: &GodotPieceColor) -> Self {
        Self::from_u8(x as u8, y as u8, player_color)
    }

    pub fn from_f32(x: f32, y: f32, player_color: &GodotPieceColor) -> Self {
        Self::from_u8(x as u8, y as u8, player_color)
    }

    pub fn from_usize(x: usize, y: usize, player_color: &GodotPieceColor) -> Self {
        Self::from_u8(x as u8, y as u8, player_color)
    }

    pub fn from_field_index(i: usize, player_color: &GodotPieceColor) -> Self {
        Self::from_u8((i % 8) as u8, (i / 8) as u8, player_color)
    }

    pub fn from_ui_vector2(vec: Vector2, square_size: f32, player_color: &GodotPieceColor) -> Self {
        Self::from_u8((vec.x / square_size) as u8, (vec.y / square_size) as u8, player_color)
    }

    pub fn from_engine_square(square: Square) -> Self {
        Self(square)
    }

    pub fn get_ui_vector2(&self, square_size: f32, player_color: &GodotPieceColor) -> Vector2 {
        let square_index = self.get_field_index(player_color);
        Vector2::new(
            (square_index % 8) as f32 * square_size,
            (square_index / 8) as f32 * square_size,
        )
    }

    pub fn get_field_index(&self, player_color: &GodotPieceColor) -> usize {
        if player_color == &GodotPieceColor::White {
            return (self.0.get_file() + (7 - self.0.get_rank()) * 8) as usize;
        }

        (7 - self.0.get_file() + self.0.get_rank() * 8) as usize
    }

    pub fn get_square(&self) -> Square {
        self.0
    }
}

impl Display for GodotSquare {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
