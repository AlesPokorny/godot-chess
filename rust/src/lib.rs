mod chess_board;
mod chess_pieces;
mod consts;
mod sounds;
mod utils;

use crate::utils::GodotPosition;
use chess_board::{GodotBoard, GodotSelectSquare};
use chess_pieces::{GodotPiece, GodotPieceColor};
use consts::{CAPTURE_SOUND_FILE_NAME, MOVE_SOUND_FILE_NAME, RESOURCES_FOLDER_PATH, SOUNDS_SUBFOLDER_PATH, START_POSITION};
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
            if mouse_button_event.get_button_index() == MouseButton::LEFT && mouse_button_event.is_pressed() {
                let click_position = GodotPosition::from_ui_vector2(mouse_button_event.get_position(), self.square_size);
                godot_print!("Turn {}", self.turn);
                godot_print!("click positin: {}", click_position);
                godot_print!("Selected i: {:?}", self.selected_piece_index);

                match self.selected_piece_index {
                    None => {
                        if let Some(piece) = self.pieces.get(click_position.to_field_index()).unwrap() {
                            if piece.bind().color == self.turn {
                                self.selected_piece_index = Some(click_position.to_field_index());
                                // mark square
                                self.select_square
                                    .set_position(click_position.to_ui_vector2(self.square_size));
                                self.select_square.set_visible(true);
                            }
                        }
                        return;
                    }
                    Some(selected_piece_index) => {
                        match self.pieces.get(click_position.to_field_index()).unwrap() {
                            Some(piece_in_field) => {
                                // Clear selection if same color
                                if piece_in_field.bind().color == self.turn {
                                    self.select_square
                                        .set_position(click_position.to_ui_vector2(self.square_size));
                                    self.selected_piece_index = Some(click_position.to_field_index());
                                    return;
                                }

                                // capture
                                let child_to_drop = piece_in_field.clone();
                                self.base_mut().remove_child(&child_to_drop);
                                self.move_piece(selected_piece_index, click_position);
                                self.play_move_sound();
                            }
                            None => {
                                // Move
                                self.move_piece(selected_piece_index, click_position);
                                self.play_capture_sound();
                            }
                        }
                    }
                }
                self.select_square.set_visible(false);
                self.selected_piece_index = None;
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
                let piece_position = GodotPosition::from_field_index(i);
                let mut piece = GodotPiece::new_alloc();
                piece.bind_mut().set_piece(kind, color, self.square_size);
                piece.set_position(piece_position.to_ui_vector2(self.square_size));
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

    fn move_piece(&mut self, from: usize, to: GodotPosition) {
        let selected_piece = self.pieces[from].as_mut().unwrap();
        selected_piece.set_position(to.to_ui_vector2(self.square_size));
        self.pieces[to.to_field_index()] = self.pieces[from].clone();
        self.pieces[from] = None;
    }

    fn play_move_sound(&mut self) {
        self.sound_move.bind_mut().player.play();
    }

    fn play_capture_sound(&mut self) {
        self.sound_capture.bind_mut().player.play();
    }
}
