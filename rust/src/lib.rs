mod chess_board;
mod chess_pieces;
mod consts;
mod engine;
mod game;
mod moves;
mod sounds;
mod square;

use godot::prelude::*;

struct GodotChess;

#[gdextension]
unsafe impl ExtensionLibrary for GodotChess {}
