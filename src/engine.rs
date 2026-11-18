use crate::{
    errors::ZeroResult,
    representations::{Board, Piece},
};

pub struct Engine {
    pub board: Board,
    pub moves: Vec<(i8, i8)>,
    pub moves_count: u8,
    pub captured: Vec<Piece>,
    pub turn: bool, // true for white false for black
}

impl Engine {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            moves: Vec::new(),
            captured: Vec::new(),
            moves_count: 0,
            turn: true,
        }
    }

    pub fn make_move(&mut self, from: i8, to: i8) -> ZeroResult<()> {
        self.can_move(from, to)?;

        let taken_ip = std::mem::take(&mut self.board.0[from as usize]);
        let cp = std::mem::replace(&mut self.board.0[to as usize], taken_ip);
        self.captured.push(cp);

        self.moves.push((from, to));

        self.moves_count += 1;

        self.turn = !self.turn;

        Ok(())
    }

    #[allow(warnings)]
    pub fn undo(&mut self) {
        if let (Some((from, to)), Some(cp)) = (self.moves.pop(), self.captured.pop()) {
            let oldp = std::mem::replace(&mut self.board.0[to as usize], cp);
            self.board.0[from as usize] = oldp;
            self.moves_count -= 1;
        }
    }
}
