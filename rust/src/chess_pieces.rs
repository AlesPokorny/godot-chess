use std::fmt::Display;

use godot::classes::{ITextureRect, Image, ImageTexture, TextureRect};
use godot::prelude::*;

use crate::consts::*;

#[derive(PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum GodotPieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
    Na,
}

impl Display for GodotPieceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Self::Pawn => "p",
            Self::Knight => "n",
            Self::Bishop => "b",
            Self::Rook => "r",
            Self::Queen => "q",
            Self::King => "k",
            Self::Na => panic!("Asking to display NA kind"),
        };
        write!(f, "{}", output)
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum GodotPieceColor {
    White,
    Black,
}

impl GodotPieceColor {
    pub fn opponent_turn(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

impl Display for GodotPieceColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Self::White => "w",
            Self::Black => "b",
        };
        write!(f, "{}", output)
    }
}

#[derive(GodotClass)]
#[class(base=TextureRect)]
pub struct GodotPiece {
    image_file_name: String,
    pub color: GodotPieceColor,
    pub kind: GodotPieceKind,
    base: Base<TextureRect>,
}

#[godot_api]
impl ITextureRect for GodotPiece {
    fn init(base: Base<TextureRect>) -> Self {
        Self {
            image_file_name: String::new(),
            color: GodotPieceColor::White,
            kind: GodotPieceKind::Na,
            base,
        }
    }
}

impl GodotPiece {
    pub fn set_piece(&mut self, kind: GodotPieceKind, color: GodotPieceColor, size: f32) {
        self.image_file_name = format!("{}{}.png", kind, color);
        self.color = color;
        self.kind = kind;

        self.base_mut().set_size(Vector2::new(size, size));
    }

    pub fn set_image(&mut self) {
        let image = Image::load_from_file(&format!("{}{}", RESOURCES_FOLDER_PATH, self.image_file_name)).unwrap();
        let texture = ImageTexture::create_from_image(&image).unwrap();
        self.base_mut().set_texture(&texture);
    }
}
