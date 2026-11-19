use super::*;

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

    pub fn make_move(&mut self, from: u32, n: u32) -> ZeroResult<()> {
        let p = &mut self.board.0[from as usize];
        p.meta.rank = from.to_rank();
        p.meta.file = from.to_file();

        self.can_move(from)?;

        let mut p = self.board.0[from as usize];
        let board = &mut self.board;
        p.move_nforward(board, n)?;

        self.moves_count += 1;

        self.turn = !self.turn;

        Ok(())
    }
}
