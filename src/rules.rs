use crate::{
    engine::Engine,
    errors::{ZeroErr, ZeroResult},
    representations::{Piece, PieceType},
};

pub const BM: [i8; 4] = [-7, 7, 9, -9];

impl Engine {
    pub fn can_move(&self, from: i8, to: i8) -> ZeroResult<()> {
        self.check_turn(from, to)?;

        let p = &self.board.0[from as usize];

        let offset = Self::calc_offset(from, to)?;

        self.moves_rules(p, offset)?;
        self.can_take(from, to)?;

        Ok(())
    }

    fn calc_offset(from: i8, to: i8) -> ZeroResult<i8> {
        Ok(to - from)
    }

    fn moves_rules(&self, p: &Piece, current: i8) -> ZeroResult<()> {
        match p.piece_type {
            PieceType::Pawn => {
                let mut moves = (8, 16);

                if !self.turn {
                    moves = (-8, -16)
                }

                let can_move;

                if p.meta.has_moved {
                    can_move = current == moves.0;
                } else {
                    can_move = current == moves.1 || current == moves.0;
                }

                if !can_move {
                    return Err(ZeroErr::Illegal {
                        at: format!("{}", current),
                        why: format!("can not move a pawn more than 2 or 1"),
                    });
                }
            }
            PieceType::Bishop => {
                let mut bm = BM;

                if !self.turn {
                    bm = BM.map(|x| x * -1)
                }

                let res = bm.iter().any(|x| *x == current);

                if !res {}
            }
            _ => {}
        }

        Ok(())
    }

    fn check_turn(&self, from: i8, to: i8) -> ZeroResult<()> {
        if self.turn && self.board.0[from as usize].color.is_black() {
            return Err(ZeroErr::Illegal {
                at: format!("({}->{})", from, to),
                why: "it's white's turn to move!".to_string(),
            });
        }

        if !self.turn && self.board.0[from as usize].color.is_white() {
            return Err(ZeroErr::Illegal {
                at: format!("({}->{})", from, to),
                why: "it's black's turn to move!".to_string(),
            });
        }

        Ok(())
    }

    fn can_take(&self, from: i8, to: i8) -> ZeroResult<()> {
        let i_p = self.board.0[from as usize];
        let f_p = self.board.0[to as usize];

        if i_p.color == f_p.color {
            return Err(ZeroErr::Illegal {
                at: format!("({} -> {})", from, to),
                why: "You can not take your own pieces".to_string(),
            });
        }

        Ok(())
    }
}
