use godot::classes::image::Format;
use godot::classes::{
    IReferenceRect, ITextureRect, Image, ImageTexture, ReferenceRect, TextureRect,
};
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
        godot_print!("Hello, world!");

        Self {
            square_size: 0.,
            base,
        }
    }

    fn ready(&mut self) {
        godot_print!("test");
        let board_size = self.square_size * 8.;
        let square_size_int = self.square_size as i32;
        let mut base = self.base_mut();
        base.set_position(Vector2::ZERO);
        base.set_size(Vector2::new(board_size, board_size));

        let mut image =
            Image::create_empty(board_size as i32, board_size as i32, false, Format::RGB8).unwrap();

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
        godot_print!("DID IT GET HERE?");
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
