use std::collections::{HashMap, HashSet};

use crate::chess_board::{GodotBoard, GodotSelectSquare, LegalMoveHelper, PromotionRect};
use crate::chess_pieces::{GodotPiece, GodotPieceColor, GodotPieceKind};
use crate::consts::{CAPTURE_SOUND_FILE_NAME, MOVE_SOUND_FILE_NAME, RESOURCES_FOLDER_PATH, SOUNDS_SUBFOLDER_PATH};
use crate::engine::ChessEngine;
use crate::moves::GodotMove;
use crate::sounds::GodotSounds;
use crate::square::GodotSquare;
use godot::classes::{INode2D, ITextureRect, InputEvent, InputEventMouseButton, Node2D};
use godot::global::MouseButton;
use godot::prelude::*;
use rustier_chess::types::square::Square;

#[derive(GodotClass)]
#[class(base=Node2D)]
struct GodotGame {
    board_background: Gd<GodotBoard>,
    engine: ChessEngine,
    legal_moves: HashMap<GodotSquare, HashSet<GodotMove>>,
    legal_move_helpers: Vec<Gd<LegalMoveHelper>>,
    pieces: [Option<Gd<GodotPiece>>; 64],
    player_color: GodotPieceColor,
    promotion_rect: Gd<PromotionRect>,
    promotion_square: Option<GodotSquare>,
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
            player_color: GodotPieceColor::White,
            promotion_rect: PromotionRect::new_alloc(),
            promotion_square: None,
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

    fn input(&mut self, input_event: Gd<InputEvent>) {
        if let Ok(mouse_button_event) = input_event.try_cast::<InputEventMouseButton>() {
            if mouse_button_event.get_button_index() == MouseButton::LEFT && mouse_button_event.is_pressed() {
                self.clear_helpers();
                let click_position =
                    GodotSquare::from_ui_vector2(mouse_button_event.get_position(), self.square_size, &self.player_color);

                if let Some(promotion_square) = self.promotion_square {
                    if click_position.get_file() != promotion_square.get_file()
                        || (promotion_square.get_rank().abs_diff(click_position.get_rank()) > 3)
                    {
                        self.selected_piece_square = None;
                        self.selected_piece_kind = None;
                        self.hide_select_square();
                        self.promotion_square = None;
                        self.promotion_rect.hide();

                        return;
                    }

                    let selected_piece_square = self.selected_piece_square.unwrap();

                    let origin_index = selected_piece_square.get_field_index(&self.player_color);
                    let destination_index = promotion_square.get_field_index(&self.player_color);

                    match self.pieces[destination_index].clone() {
                        // Capture
                        Some(piece_in_field) => {
                            self.base_mut().remove_child(&piece_in_field);
                            self.play_capture_sound();
                        }
                        // Move
                        None => {
                            self.play_move_sound();
                        }
                    }
                    let child_to_drop_origin = self.pieces[origin_index].clone().unwrap();
                    self.base_mut().remove_child(&child_to_drop_origin);
                    self.pieces[origin_index] = None;

                    let (new_piece_kind, promotion_piece_n) = match promotion_square.get_rank().abs_diff(click_position.get_rank()) {
                        0 => {
                            (GodotPieceKind::Queen, 0)
                        }
                        1 => {
                            (GodotPieceKind::Knight, 3)
                        }
                        2 => {
                            (GodotPieceKind::Rook, 1)
                        }
                        3 => {
                            (GodotPieceKind::Bishop, 2)
                        }
                        _ => panic!("Should be able to get here"),
                    };

                    self.init_piece(new_piece_kind, self.turn, &promotion_square);

                    let legal_move = GodotMove::from_origin_destination_and_promotion(
                        &selected_piece_square,
                        &promotion_square,
                        promotion_piece_n,
                    );

                    self.engine.play_move(&legal_move);
                    self.end_turn();
                    return;
                }

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
                                        if legal_move.is_promotion() {
                                            self.promotion_rect.bind_mut().show(&click_position, self.square_size);
                                            self.promotion_square = Some(click_position);
                                            return;
                                        }

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

                                        if legal_move.is_promotion() {
                                            self.promotion_rect.bind_mut().show(&click_position, self.square_size);
                                            self.promotion_square = Some(click_position);
                                            return;
                                        }

                                        // castling
                                        if legal_move.is_castling() {
                                            self.move_rook_for_castling(&legal_move);
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
                self.end_turn();
            }
        }
    }
}

#[godot_api]
impl GodotGame {
    #[func]
    fn custom_ready(&mut self) {
        self.init_board();
        self.init_select_square();
        self.init_pieces();
        self.init_promotion_rect();
        self.init_sounds();
    }

    #[func]
    fn start(&mut self, color: String) {
        self.player_color = if color == "white" {
            GodotPieceColor::White
        } else {
            GodotPieceColor::Black
        };
        self.custom_ready();
    }

    #[func]
    fn start_from_fen(&mut self, fen: String) {
        if let Ok(engine) = ChessEngine::from_fen(&fen) {
            self.engine = engine;
            self.player_color = self.engine.get_turn();
            self.turn = self.player_color;
            self.custom_ready();
        }
    }

    #[func]
    fn check_fen_string(&mut self, fen: String) -> bool {
        ChessEngine::from_fen(&fen).is_ok()
    }

    fn init_board(&mut self) {
        let mut board = GodotBoard::new_alloc();
        board.bind_mut().set_square_size(self.square_size);
        board.bind_mut().ready();
        self.base_mut().add_child(&board);
        self.board_background = board;
    }

    fn init_select_square(&mut self) {
        let mut select_square = GodotSelectSquare::new_alloc();
        select_square.set_size(Vector2::new(self.square_size, self.square_size));
        self.base_mut().add_child(&select_square);
        self.select_square = select_square;
    }

    fn init_promotion_rect(&mut self) {
        let mut promotion_rect = PromotionRect::new_alloc();
        promotion_rect.bind_mut().set(self.player_color, self.square_size);
        self.base_mut().add_child(&promotion_rect);
        self.promotion_rect = promotion_rect;
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
        let pieces = self.engine.get_pieces_per_square(&self.player_color);
        for (square, color, kind) in pieces {
            self.init_piece(kind, color, &square);
        }
    }

    fn init_piece(&mut self, kind: GodotPieceKind, color: GodotPieceColor, square: &GodotSquare) {
        let mut piece = GodotPiece::new_alloc();
        piece.bind_mut().set_piece(kind, color, self.square_size);
        piece.set_position(square.get_ui_vector2(self.square_size, &self.player_color));
        piece.bind_mut().set_image();

        self.base_mut().add_child(&piece);
        self.pieces[square.get_field_index(&self.player_color)] = Some(piece);
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

    fn move_rook_for_castling(&mut self, castling_move: &GodotMove) {
        let king_destination = castling_move.get_destination();
        let king_destination_square = king_destination.get_square();
        let (rook_origin_file, rook_destination_file) = if king_destination_square.get_file() == 2 {
            // long
            (0, 3)
        } else {
            // Short
            (7, 5)
        };
        let rook_rank = king_destination_square.get_rank();

        let rook_origin = GodotSquare::from_engine_square(Square::new(rook_rank * 8 + rook_origin_file));
        let rook_destination = GodotSquare::from_engine_square(Square::new(rook_rank * 8 + rook_destination_file));

        self.move_piece(&rook_origin, &rook_destination);
    }

    fn end_turn(&mut self) {
        self.hide_select_square();
        self.legal_moves.clear();
        self.turn = self.turn.opponent_turn();
        self.selected_piece_square = None;
        self.selected_piece_kind = None;
        self.promotion_rect.hide();
    }
}
