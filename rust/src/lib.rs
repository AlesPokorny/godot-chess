mod chess_board;
mod chess_pieces;
mod consts;
mod utils;

use godot::prelude::*;

struct GodotChess;

#[gdextension]
unsafe impl ExtensionLibrary for GodotChess {}
