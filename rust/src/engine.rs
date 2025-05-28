use std::collections::{HashMap, HashSet};

use godot::global::godot_print;
use rustier_chess::board::Board;
use rustier_chess::bots::bot::Bot;
use rustier_chess::moves::move_mask_gen::MoveGenMasks;
use rustier_chess::utils::zobrist::ZobristHasher;

use crate::consts::ENGINE_MOVE_FOLDER_PATH;
use crate::moves::GodotMove;
use crate::square::GodotSquare;

pub struct ChessEngine {
    board: Board,
    bot: Bot,
    hasher: ZobristHasher,
    move_gen_mask: MoveGenMasks,
}

impl Default for ChessEngine {
    fn default() -> Self {
        let hasher = ZobristHasher::load();

        Self {
            board: Board::new(&hasher),
            bot: Bot::default(),
            hasher,
            // legal_moves: HashSet::with_capacity(218),  // Max legal moves in a chess position
            move_gen_mask: MoveGenMasks::load_from_path(ENGINE_MOVE_FOLDER_PATH),
        }
    }
}

impl ChessEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_legal_moves(&mut self) -> HashMap<GodotSquare, HashSet<GodotMove>> {
        let mut output: HashMap<GodotSquare, HashSet<GodotMove>> = HashMap::new();
        for engine_move in self.board.get_legal_moves(&self.move_gen_mask, &self.hasher) {
            let from = GodotSquare::from_engine_square(engine_move.get_origin());

            match output.get_mut(&from) {
                Some(moves) => {
                    moves.insert(GodotMove::from_engine_move(engine_move));
                }
                None => {
                    output.insert(from, HashSet::from_iter([GodotMove::from_engine_move(engine_move)]));
                }
            }
        }

        output
    }

    pub fn play_move(&mut self, legal_move: &GodotMove) {
        self.board.make_move(&legal_move.get_engine_move(), &self.hasher);
        godot_print!("{}", self.board);
    }
}
