use std::collections::{HashMap, HashSet};

use crate::chess_board::{GodotBoard, GodotSelectSquare, LegalMoveHelper};
use crate::chess_pieces::{GodotPiece, GodotPieceColor, GodotPieceKind};
use crate::consts::{
    CAPTURE_SOUND_FILE_NAME, MOVE_SOUND_FILE_NAME, RESOURCES_FOLDER_PATH, SOUNDS_SUBFOLDER_PATH, START_POSITION,
};
use crate::engine::ChessEngine;
use crate::moves::GodotMove;
use crate::sounds::GodotSounds;
use crate::square::GodotSquare;
use godot::classes::{INode2D, ITextureRect, InputEvent, InputEventMouseButton, Node2D};
use godot::global::MouseButton;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct GodotGame {
    board_background: Gd<GodotBoard>,
    engine: ChessEngine,
    legal_moves: HashMap<GodotSquare, HashSet<GodotMove>>,
    legal_move_helpers: Vec<Gd<LegalMoveHelper>>,
    pieces: [Option<Gd<GodotPiece>>; 64],
    player_color: GodotPieceColor,
    select_square: Gd<GodotSelectSquare>,
    selected_piece_kind: Option<GodotPieceKind>,
    selected_piece_square: Option<GodotSquare>,
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
            board_background: GodotBoard::new_alloc(),
            engine: ChessEngine::new(),
            legal_moves: HashMap::with_capacity(16),
            legal_move_helpers: Vec::with_capacity(20),
            pieces: [const { None }; 64],
            player_color: GodotPieceColor::White, // TODO: Change this using menu
            select_square: GodotSelectSquare::new_alloc(),
            selected_piece_kind: None,
            selected_piece_square: None,
            sound_move: GodotSounds::empty(),
            sound_capture: GodotSounds::empty(),
            square_size: 100.,
            turn: GodotPieceColor::White,
            base,
        }
    }

    fn ready(&mut self) {
        self.init_board();
        self.init();
        self.init_pieces();
        self.init_sounds();
    }

    fn input(&mut self, input_event: Gd<InputEvent>) {
        if let Ok(mouse_button_event) = input_event.try_cast::<InputEventMouseButton>() {
            if mouse_button_event.get_button_index() == MouseButton::LEFT && mouse_button_event.is_pressed() {
                self.clear_helpers();
                let click_position =
                    GodotSquare::from_ui_vector2(mouse_button_event.get_position(), self.square_size, &self.player_color);
                godot_print!("Turn {}", self.turn);
                godot_print!("click position: {}", click_position);
                godot_print!("Selected i: {:?}", self.selected_piece_square);

                match self.selected_piece_square {
                    None => {
                        if let Some(piece) = self.pieces.get(click_position.get_field_index(&self.player_color)).unwrap() {
                            if piece.bind().color == self.turn {
                                self.selected_piece_kind = Some(piece.bind().kind);
                                self.selected_piece_square = Some(click_position);
                                // mark square
                                self.move_select_square(&click_position);
                                // get move helpers
                                if self.legal_moves.is_empty() {
                                    self.legal_moves = self.engine.get_legal_moves();
                                }
                                // create helpers
                                if let Some(helpers) = self.legal_moves.get(&click_position) {
                                    self.draw_helpers(helpers.clone());
                                }
                            }
                        }
                        return;
                    }
                    Some(selected_piece_square) => {
                        match self.pieces.get(click_position.get_field_index(&self.player_color)).unwrap() {
                            Some(piece_in_field) => {
                                // Change selection if same color
                                if piece_in_field.bind().color == self.turn {
                                    self.move_select_square(&click_position);
                                    self.selected_piece_square = Some(click_position);
                                    if let Some(helpers) = self.legal_moves.get(&click_position) {
                                        self.draw_helpers(helpers.clone());
                                    }
                                    return;
                                }

                                match self.get_legal_move_from_origin_and_destination(&selected_piece_square, &click_position) {
                                    // Found move
                                    Some(legal_move) => {
                                        // capture
                                        self.engine.play_move(&legal_move);
                                        let child_to_drop = piece_in_field.clone();
                                        self.base_mut().remove_child(&child_to_drop);
                                        self.move_piece(&selected_piece_square, &click_position);
                                        self.play_capture_sound();
                                    }
                                    // Did not find move
                                    None => {
                                        self.hide_select_square();
                                        return;
                                    }
                                }
                            }
                            None => {
                                // Move
                                match self.get_legal_move_from_origin_and_destination(&selected_piece_square, &click_position) {
                                    // Found move
                                    Some(legal_move) => {
                                        // is en passant capture
                                        if let Some(en_passant_square) = self.engine.board.state.en_passant {
                                            if GodotSquare::from_engine_square(en_passant_square) == click_position
                                                && self.selected_piece_kind.unwrap() == GodotPieceKind::Pawn
                                            {
                                                let capture_square_index = if self.player_color == self.turn {
                                                    click_position.get_field_index(&self.player_color) + 8
                                                } else {
                                                    click_position.get_field_index(&self.player_color) - 8 
                                                };
                                                let child_to_drop = self.pieces[capture_square_index].clone().unwrap();
                                                self.base_mut().remove_child(&child_to_drop);
                                                self.pieces[capture_square_index] = None;
                                            }
                                        }

                                        self.engine.play_move(&legal_move);
                                        self.move_piece(&selected_piece_square, &click_position);
                                        self.play_move_sound();
                                    }
                                    // Did not find move
                                    None => {
                                        self.hide_select_square();
                                        return;
                                    }
                                }
                            }
                        }
                    }
                }
                self.hide_select_square();
                self.legal_moves.clear();
                self.turn = self.turn.opponent_turn();
                self.selected_piece_square = None;
                self.selected_piece_kind = None;
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
        self.board_background = board;
    }

    fn init(&mut self) {
        let mut select_square = GodotSelectSquare::new_alloc();
        select_square.set_size(Vector2::new(self.square_size, self.square_size));
        self.base_mut().add_child(&select_square);
        self.select_square = select_square;
    }

    fn hide_select_square(&mut self) {
        self.select_square.set_visible(false);
    }

    fn move_select_square(&mut self, to: &GodotSquare) {
        self.select_square
            .set_position(to.get_ui_vector2(self.square_size, &self.player_color));
        self.select_square.set_visible(true);
    }

    fn init_pieces(&mut self) {
        for (i, entry) in START_POSITION.into_iter().enumerate() {
            if let Some((color, kind)) = entry {
                let piece_position = GodotSquare::from_field_index(i, &self.player_color);
                let mut piece = GodotPiece::new_alloc();
                piece.bind_mut().set_piece(kind, color, self.square_size);
                piece.set_position(piece_position.get_ui_vector2(self.square_size, &self.player_color));
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

    fn draw_helpers(&mut self, helpers: HashSet<GodotMove>) {
        for helper_move in helpers {
            // only draw one helper for promotion, even though those are 4 separate moves
            if helper_move.is_promotion() && helper_move.get_promotion_piece() != 0 {
                continue;
            }

            let helper_position = helper_move.get_destination();
            let mut helper = LegalMoveHelper::new_alloc();
            helper.bind_mut().create(
                helper_position.get_ui_vector2(self.square_size, &self.player_color),
                self.square_size,
            );
            self.base_mut().add_child(&helper);
            self.legal_move_helpers.push(helper);
        }
    }

    fn clear_helpers(&mut self) {
        while let Some(helper) = self.legal_move_helpers.pop() {
            self.base_mut().remove_child(&helper);
        }
    }

    fn move_piece(&mut self, from: &GodotSquare, to: &GodotSquare) {
        let from_index = from.get_field_index(&self.player_color);
        let selected_piece = self.pieces[from_index].as_mut().unwrap();
        selected_piece.set_position(to.get_ui_vector2(self.square_size, &self.player_color));
        self.pieces[to.get_field_index(&self.player_color)] = self.pieces[from_index].clone();
        self.pieces[from_index] = None;
    }

    fn play_move_sound(&mut self) {
        self.sound_move.bind_mut().player.play();
    }

    fn play_capture_sound(&mut self) {
        self.sound_capture.bind_mut().player.play();
    }

    fn get_legal_move_from_origin_and_destination(&self, from: &GodotSquare, to: &GodotSquare) -> Option<GodotMove> {
        match self.legal_moves.get(from) {
            Some(legal_moves) => legal_moves
                .iter()
                .filter(|legal_move| legal_move.get_destination() == *to)
                .nth(0)
                .copied(),
            None => None,
        }
    }
}
