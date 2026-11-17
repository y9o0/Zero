use crate::{
    errors::{ZeroErr, ZeroResult},
    representations::{Board, Color, Piece, PieceType},
};

pub struct Engine {
    pub board: Board,
    pub moves: Vec<(u8, u8)>,
    pub team: Team,
    pub moves_count: u8,
    pub captured: Vec<Piece>,
}

pub struct Team {
    white: Moved,
    black: Moved,
}

impl Team {
    pub fn new() -> Self {
        Self {
            white: Moved::new(),
            black: Moved::new(),
        }
    }
}

#[derive(Debug)]
pub struct Moved {
    pawn: bool,
    rook: bool,
    king: bool,
}

impl Moved {
    pub fn new() -> Self {
        Self {
            pawn: false,
            rook: false,
            king: false,
        }
    }
}

impl Engine {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            moves: Vec::new(),
            team: Team::new(),
            captured: Vec::new(),
            moves_count: 0,
        }
    }

    pub fn make_move(&mut self, from: u8, to: u8) -> ZeroResult<()> {
        let fp = self.board.0[to as usize];
        let ip = self.board.0[from as usize];

        match (ip.piece_type, ip.color) {
            (PieceType::Pawn, Color::White) => self.team.white.pawn = true,
            (PieceType::King, Color::White) => self.team.white.king = true,
            (PieceType::Rook, Color::White) => self.team.white.rook = true,

            (PieceType::Pawn, Color::Black) => self.team.black.pawn = true,
            (PieceType::King, Color::Black) => self.team.black.king = true,
            (PieceType::Rook, Color::Black) => self.team.black.rook = true,
            _ => {}
        }

        if self.moves_count == 0 && !ip.piece_type.is_pawn()
            || !ip.piece_type.is_knight() && !ip.color.is_white() && fp.piece_type.is_empty()
        {
            return Err(ZeroErr::Illegal {
                at: format!("({} -> {})", from, to),
                why: format!(
                    "you can not move a '{}' before moving either a pawn or a knight, and white has to move first, and you can not move to unempty square",
                    &ip.piece_type
                ),
            });
        }

        let taken_ip = std::mem::take(&mut self.board.0[from as usize]);
        let cp = std::mem::replace(&mut self.board.0[to as usize], taken_ip);
        self.captured.push(cp);

        self.moves.push((from, to));

        self.moves_count += 1;

        Ok(())
    }

    pub fn undo(&mut self) {
        if let (Some((from, to)), Some(cp)) = (self.moves.pop(), self.captured.pop()) {
            let oldp = std::mem::replace(&mut self.board.0[to as usize], cp);
            self.board.0[from as usize] = oldp;
            self.moves_count -= 1;
        }
    }
}
