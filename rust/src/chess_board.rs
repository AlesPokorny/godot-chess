use godot::classes::image::Format;
use godot::classes::{
    IReferenceRect, ITextureRect, Image, ImageTexture, InputEvent, InputEventMouseButton,
    ReferenceRect, TextureRect,
};
use godot::global::MouseButton;
use godot::prelude::*;

use crate::chess_pieces::GodotPiece;
use crate::consts::*;
use crate::utils::{get_click_col_row, get_vec_from_col_row};

#[derive(GodotClass)]
#[class(base=TextureRect)]
pub struct ChessBoard {
    pieces: [Option<Gd<GodotPiece>>; 64],
    square_size: f32,
    select_square: Gd<SelectSquare>,
    base: Base<TextureRect>,
}

#[godot_api]
impl ITextureRect for ChessBoard {
    fn init(base: Base<TextureRect>) -> Self {
        godot_print!("Hello, world!"); // Prints to the Godot console

        let pieces: [Option<Gd<GodotPiece>>; 64] = [const { None }; 64];

        Self {
            pieces,
            square_size: 0.,
            select_square: SelectSquare::new_alloc(),
            base,
        }
    }

    fn ready(&mut self) {
        let board_size = self.base().get_size();
        self.square_size = board_size.x.min(board_size.y / 8.);
        self.add_select_square();
        self.init_pieces();

        let square_size = board_size.x as i32 / 8;
        let mut image = Image::create_empty(
            board_size.x as i32,
            board_size.y as i32,
            false,
            Format::RGB8,
        )
        .unwrap();
        godot_print!("image size: {}", image.get_size());
        image.fill(Color::BLACK);

        let square_size_vec = Vector2i {
            x: square_size,
            y: square_size,
        };
        let light_color = Color::from_html(LIGHT_SQUARE_COLOR).unwrap();
        let darkt_color = Color::from_html(DARK_SQUARE_COLOR).unwrap();
        for row in 0..8 {
            let x = row * square_size;
            for column in 0..8 {
                let color = if (row + column) % 2 == 0 {
                    light_color
                } else {
                    darkt_color
                };
                image.fill_rect(
                    Rect2i {
                        position: Vector2i {
                            x,
                            y: column * square_size,
                        },
                        size: square_size_vec,
                    },
                    color,
                );
            }
        }

        let texture = ImageTexture::create_from_image(&image).unwrap();
        self.base_mut().set_texture(&texture);
    }

    fn process(&mut self, delta: f64) {}

    fn input(&mut self, input_event: Gd<InputEvent>) {
        if let Ok(mouse_button_event) = input_event.try_cast::<InputEventMouseButton>() {
            if mouse_button_event.get_button_index() == MouseButton::LEFT
                && mouse_button_event.is_pressed()
            {
                let (col, row) = get_click_col_row(mouse_button_event.get_position(), 100.);

                self.select_square
                    .set_position(get_vec_from_col_row(col, row, 100.));
                self.select_square.set_visible(true);
                godot_print!("click row {}, col {}", row, col);
            }
        }
    }
}

impl ChessBoard {
    fn add_select_square(&mut self) {
        let mut select_square = SelectSquare::new_alloc();
        select_square.set_size(Vector2::new(self.square_size, self.square_size));
        self.base_mut().add_child(&select_square);
        self.select_square = select_square;
    }

    fn init_pieces(&mut self) {
        for (i, entry) in START_POSITION.into_iter().enumerate() {
            if let Some((color, kind)) = entry {
                godot_print!("workiing on: {}{}", kind, color);
                let mut piece = GodotPiece::new_alloc();
                piece.bind_mut().set_piece(kind, color, self.square_size);
                let piece_position =
                    get_vec_from_col_row((i % 8) as u16, (i / 8) as u16, self.square_size);
                piece.set_position(piece_position);
                piece.bind_mut().set_image();

                self.base_mut().add_child(&piece);
                self.pieces[i] = Some(piece);
            }
        }
    }
}

#[derive(GodotClass)]
#[class(base=ReferenceRect)]
pub struct SelectSquare {
    base: Base<ReferenceRect>,
}

#[godot_api]
impl IReferenceRect for SelectSquare {
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
