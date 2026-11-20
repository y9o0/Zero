use crate::representations::MoveRep;

use super::*;

pub enum MoveType {
    Forward(u32),
    Left(u32),
    Right(u32),
}

pub struct Engine {
    pub board: Board,
    pub moves_count: u8,

    #[allow(warnings)] // I will work on it tomorrow
    pub captured: Vec<Piece>,

    pub turn: bool, // true for white false for black
}

impl Engine {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            captured: Vec::new(),
            moves_count: 0,
            turn: true,
        }
    }

    pub fn make_move(&mut self, from: MoveRep, move_type: MoveType) -> ZeroResult<()> {
        let from = from.to_index();

        let mut p = self.board.0[from as usize];
        p.meta.rank = from.to_rank();
        p.meta.file = from.to_file();

        self.can_move(from)?;

        let board = &mut self.board;

        match move_type {
            MoveType::Forward(n) => p.move_nforward(board, n)?,
            MoveType::Left(n) => p.move_nleft(board, n)?,
            MoveType::Right(n) => p.move_nright(board, n)?,
        }

        self.moves_count += 1;

        self.turn = !self.turn;

        Ok(())
    }
}
