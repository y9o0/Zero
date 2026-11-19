use crate::{
    engine::{core::Engine, moves::MoveForward},
    errors::{ZeroErr, ZeroResult},
    representations::{Board, File, Piece, PieceType, Ranks, to_index},
};

pub mod core;
pub mod moves;
pub mod rules;
