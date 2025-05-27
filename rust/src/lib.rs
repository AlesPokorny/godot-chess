mod chess_board;
mod chess_pieces;
mod consts;
mod sounds;
mod utils;

use crate::utils::{get_click_col_row, get_vec_from_col_row};
use chess_board::{GodotBoard, GodotSelectSquare};
use chess_pieces::{GodotPiece, GodotPieceColor};
use consts::{
    CAPTURE_SOUND_FILE_NAME, MOVE_SOUND_FILE_NAME, RESOURCES_FOLDER_PATH, SOUNDS_SUBFOLDER_PATH,
    START_POSITION,
};
use godot::classes::{INode2D, ITextureRect, InputEvent, InputEventMouseButton, Node2D};
use godot::global::MouseButton;
use godot::prelude::*;
use sounds::GodotSounds;

struct GodotChess;

#[gdextension]
unsafe impl ExtensionLibrary for GodotChess {}

#[derive(GodotClass)]
#[class(base=Node2D)]
struct GodotGame {
    board: Gd<GodotBoard>,
    pieces: [Option<Gd<GodotPiece>>; 64],
    select_square: Gd<GodotSelectSquare>,
    selected_piece_index: Option<usize>,
    sound_move: Gd<GodotSounds>,
    sound_capture: Gd<GodotSounds>,
    square_size: f32,
    turn: GodotPieceColor,
    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for GodotGame {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            board: GodotBoard::new_alloc(),
            pieces: [const { None }; 64],
            select_square: GodotSelectSquare::new_alloc(),
            selected_piece_index: None,
            sound_move: GodotSounds::empty(),
            sound_capture: GodotSounds::empty(),
            square_size: 100.,
            turn: GodotPieceColor::White,
            base,
        }
    }

    fn ready(&mut self) {
        self.init_board();
        self.init_select_square();
        self.init_pieces();
        self.init_sounds();
    }

    fn input(&mut self, input_event: Gd<InputEvent>) {
        if let Ok(mouse_button_event) = input_event.try_cast::<InputEventMouseButton>() {
            if mouse_button_event.get_button_index() == MouseButton::LEFT
                && mouse_button_event.is_pressed()
            {
                self.sound_capture.bind_mut().play();
                let (col, row) = get_click_col_row(mouse_button_event.get_position(), 100.);
                godot_print!("Turn {}", self.turn);
                godot_print!("click row {}, col {}", row, col);
                godot_print!("Selected i: {:?}", self.selected_piece_index);

                if self.selected_piece_index.is_none() {
                    // select piece
                    if let Some(piece) = self.pieces.get((col + row * 8) as usize).unwrap() {
                        if piece.bind().color == self.turn {
                            self.selected_piece_index = Some((col + row * 8) as usize);
                            // mark square
                            self.select_square
                                .set_position(get_vec_from_col_row(col, row, 100.));
                            self.select_square.set_visible(true);
                        }
                    }
                    return;
                }
                if let Some(selected_piece_index) = self.selected_piece_index {
                    if let Some(piece) = self.pieces.get_mut(selected_piece_index).unwrap() {
                        // capture

                        // move
                        piece.bind_mut().move_to_col_row(col, row, self.square_size);
                    }
                    self.select_square.set_visible(false);
                    self.selected_piece_index = None;
                }

                self.turn = self.turn.opponent_turn();
            }
        }
    }
}

impl GodotGame {
    fn init_board(&mut self) {
        let mut board = GodotBoard::new_alloc();
        board.bind_mut().set_square_size(self.square_size);
        board.bind_mut().ready();
        self.base_mut().add_child(&board);
        self.board = board;
    }

    fn init_select_square(&mut self) {
        let mut select_square = GodotSelectSquare::new_alloc();
        select_square.set_size(Vector2::new(self.square_size, self.square_size));
        self.base_mut().add_child(&select_square);
        self.select_square = select_square;
    }

    fn init_pieces(&mut self) {
        for (i, entry) in START_POSITION.into_iter().enumerate() {
            if let Some((color, kind)) = entry {
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

    fn init_sounds(&mut self) {
        let sound_move = GodotSounds::from_path(&format!(
            "{}{}{}",
            RESOURCES_FOLDER_PATH, SOUNDS_SUBFOLDER_PATH, MOVE_SOUND_FILE_NAME
        ));
        self.base_mut().add_child(&sound_move.bind().player);
        self.sound_move = sound_move;
        let sound_capture = GodotSounds::from_path(&format!(
            "{}{}{}",
            RESOURCES_FOLDER_PATH, SOUNDS_SUBFOLDER_PATH, CAPTURE_SOUND_FILE_NAME
        ));
        self.base_mut().add_child(&sound_capture.bind().player);
        self.sound_capture = sound_capture;
    }
}
