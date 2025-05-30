use core::f64;

use godot::classes::image::Format;
use godot::classes::{
    ColorRect, IColorRect, IPolygon2D, IReferenceRect, ITextureRect, Image, ImageTexture, Polygon2D, ReferenceRect, TextureRect,
};
use godot::prelude::*;

use crate::chess_pieces::{GodotPiece, GodotPieceColor, GodotPieceKind};
use crate::consts::*;
use crate::square::GodotSquare;

#[derive(GodotClass)]
#[class(base=TextureRect)]
pub struct GodotBoard {
    square_size: f32,
    base: Base<TextureRect>,
}

#[godot_api]
impl ITextureRect for GodotBoard {
    fn init(base: Base<TextureRect>) -> Self {
        Self { square_size: 0., base }
    }

    fn ready(&mut self) {
        let board_size = self.square_size * 8.;
        let square_size_int = self.square_size as i32;
        let mut base = self.base_mut();
        base.set_position(Vector2::ZERO);
        base.set_size(Vector2::new(board_size, board_size));

        let mut image = Image::create_empty(board_size as i32, board_size as i32, false, Format::RGB8).unwrap();

        let square_size_vec = Vector2i {
            x: square_size_int,
            y: square_size_int,
        };
        let light_color = Color::from_html(LIGHT_SQUARE_COLOR).unwrap();
        image.fill(Color::from_html(DARK_SQUARE_COLOR).unwrap());
        for row in 0..8 {
            let x = row * square_size_int;
            for column in 0..8 {
                if (row + column) % 2 == 1 {
                    continue;
                }
                image.fill_rect(
                    Rect2i {
                        position: Vector2i {
                            x,
                            y: column * square_size_int,
                        },
                        size: square_size_vec,
                    },
                    light_color,
                );
            }
        }

        let texture = ImageTexture::create_from_image(&image).unwrap();
        base.set_texture(&texture);
    }
}

impl GodotBoard {
    pub fn set_square_size(&mut self, size: f32) {
        self.square_size = size;
    }
}

#[derive(GodotClass)]
#[class(base=ReferenceRect)]
pub struct GodotSelectSquare {
    base: Base<ReferenceRect>,
}

#[godot_api]
impl IReferenceRect for GodotSelectSquare {
    fn init(base: Base<ReferenceRect>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        let size = self.base().get_size().x;
        self.base_mut()
            .set_border_color(Color::from_html(SELECT_BORDER_COLOR).unwrap());
        self.base_mut().set_border_width(size / 10.);
        self.base_mut().set_visible(false);
        self.base_mut().set_editor_only(false);
    }
}

#[derive(GodotClass)]
#[class(base=Polygon2D)]
pub struct LegalMoveHelper {
    base: Base<Polygon2D>,
}

#[godot_api]
impl IPolygon2D for LegalMoveHelper {
    fn init(base: Base<Polygon2D>) -> Self {
        Self { base }
    }
}

impl LegalMoveHelper {
    const ANGLE_IN_RAD: f32 = ((f64::consts::PI / 180.) * 30.) as f32;
    const SQUARE_FRACTION: f32 = 1. / 5.;

    pub fn create(&mut self, position: Vector2, square_size: f32) {
        let width = square_size * Self::SQUARE_FRACTION;
        let height = Self::ANGLE_IN_RAD.cos() * width;
        let x_offset = Self::ANGLE_IN_RAD.sin() * width;

        let center_x = position.x + square_size / 2.;
        let center_y = position.y + square_size / 2.;

        let points = [
            Vector2::new(center_x - x_offset, center_y - height),
            Vector2::new(center_x + x_offset, center_y - height),
            Vector2::new(center_x + width, center_y),
            Vector2::new(center_x + x_offset, center_y + height),
            Vector2::new(center_x - x_offset, center_y + height),
            Vector2::new(center_x - width, center_y),
        ];

        self.base_mut().set_polygon(&PackedVector2Array::from_iter(points));
        self.base_mut().set_color(Color::from_html(LEGAL_MOVE_HELPER_COLOR).unwrap());

        self.base_mut().set_visible(true);
    }
}

#[derive(GodotClass)]
#[class(base=ColorRect)]
pub struct PromotionRect {
    color: GodotPieceColor,
    pieces: [Gd<GodotPiece>; 4],
    base: Base<ColorRect>,
}

#[godot_api]
impl IColorRect for PromotionRect {
    fn init(base: Base<ColorRect>) -> Self {
        Self {
            color: GodotPieceColor::White,
            pieces: [
                GodotPiece::new_alloc(),
                GodotPiece::new_alloc(),
                GodotPiece::new_alloc(),
                GodotPiece::new_alloc(),
            ],
            base,
        }
    }
}

impl PromotionRect {
    const PROMOTION_PIECES: [GodotPieceKind; 4] = [
        GodotPieceKind::Queen,
        GodotPieceKind::Knight,
        GodotPieceKind::Rook,
        GodotPieceKind::Bishop,
    ];
    pub fn set(&mut self, color: GodotPieceColor, square_size: f32) {
        self.color = color;
        self.base_mut().set_size(Vector2::new(square_size, square_size * 4.));
        self.base_mut().set_color(Color::GRAY);
        self.add_pieces(square_size);
        self.base_mut().set_visible(false);
    }
    fn add_pieces(&mut self, square_size: f32) {
        for (i, piece_kind) in Self::PROMOTION_PIECES.into_iter().enumerate() {
            let mut piece = GodotPiece::new_alloc();
            piece.bind_mut().set_piece(piece_kind, self.color, square_size);
            piece.bind_mut().set_image();
            piece.set_visible(false);
            piece.set_position(Vector2::new(0., square_size * i as f32));
            self.base_mut().add_child(&piece);
            self.pieces[i] = piece;
        }
    }

    pub fn show(&mut self, square: &GodotSquare, square_size: f32) {
        self.base_mut().set_visible(true);
        let color = self.color;
        let rectangle_position = square.get_ui_vector2(square_size, &color);
        self.base_mut().set_position(rectangle_position);

        for piece in self.pieces.iter_mut() {
            piece.set_visible(true)
        }
    }
}
