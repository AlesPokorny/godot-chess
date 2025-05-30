use rustier_chess::moves::moves_utils::Move;

use crate::square::GodotSquare;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct GodotMove(Move);

impl GodotMove {
    pub fn from_engine_move(engine_move: Move) -> Self {
        Self(engine_move)
    }

    pub fn from_origin_destination_and_promotion(
        origin: &GodotSquare,
        destination: &GodotSquare,
        promotion_piece_n: usize,
    ) -> Self {
        let mut engine_move = Move::from_origin_and_destination(&destination.get_square(), &origin.get_square());
        engine_move.set_promotion(promotion_piece_n);
        Self::from_engine_move(engine_move)
    }

    pub fn get_destination(&self) -> GodotSquare {
        GodotSquare::from_engine_square(self.0.get_destination())
    }

    pub fn is_promotion(&self) -> bool {
        self.0.special_move() == 1
    }

    pub fn get_promotion_piece(&self) -> usize {
        self.0.get_promotion_piece()
    }

    pub fn get_engine_move(&self) -> Move {
        self.0
    }

    pub fn is_castling(&self) -> bool {
        self.0.special_move() == 3
    }
}
