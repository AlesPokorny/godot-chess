use core::f64;

use godot::classes::image::Format;
use godot::classes::{IPolygon2D, IReferenceRect, ITextureRect, Image, ImageTexture, Polygon2D, ReferenceRect, TextureRect};
use godot::prelude::*;

use crate::consts::*;

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
        godot_print!("test");
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
