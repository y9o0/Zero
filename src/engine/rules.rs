use std::cmp::Ordering::Greater;

use super::*;

pub trait CanMove {
    fn can_move_nforwards(&self, n: u32, board: &Board) -> ZeroResult<()>;
    fn can_move_nleft(&self, n: u32, board: &Board) -> ZeroResult<()>;
    fn can_move_nright(&self, n: u32, board: &Board) -> ZeroResult<()>;
    fn can_move_nback(&self, n: u32, board: &Board) -> ZeroResult<()>;
}

fn same_color_error(p: &Piece, p2: &Piece) -> ZeroResult<()> {
    if p.color == p2.color {
        return Err(ZeroErr::Illegal {
            at: format!("(file : {}, rank : {})", p.meta.file, p.meta.rank),
            why: format!(
                "the '{}' is blooked by '{}' from the same color",
                p.piece_type, p2.piece_type
            ),
        });
    }

    Ok(())
}

fn checked_sub_move(p: &Piece, n: u32) -> ZeroResult<u32> {
    p.meta
        .file
        .checked_sub(n)
        .ok_or_else(|| ZeroErr::Unexpected {
            message: format!(
                "You have went so far beyond the limits of the board! ({} at {})",
                p.piece_type,
                p.as_rep()
            ),
        })
}

fn have_capture_error(p: &Piece, p2: &Piece) -> ZeroResult<()> {
    if !p2.piece_type.is_empty() && p.color != p2.color {
        return Err(ZeroErr::Illegal {
            at: format!("(file : {}, rank : {})", p.meta.file, p.meta.rank),
            why: "you have to capture the piece, you can not move over it".to_string(),
        });
    }

    Ok(())
}

fn beyond_limits_error(n: u32, c: u32) -> ZeroResult<()> {
    match n.cmp(&c) {
        Greater => {
            return Err(ZeroErr::Unexpected {
                message: format!(
                    "You have went so far beyond the limits of the board! ({} >! {})",
                    n, c
                ),
            });
        }
        _ => {}
    }

    Ok(())
}

#[macro_export]
macro_rules! otherthan {
    ($p:ident ~ $( $v:path )|*  > $m:expr  ) => {{
        let p: &$crate::representations::Piece = &$p;

         match p.piece_type {
             $( $v )|* => {}
             _ => {
                 return Err($crate::errors::ZeroErr::Illegal {
                     at: format!("{}", p.as_rep()),
                     why: $m.to_string(),
                 });
             }
         }
    }};
}

impl CanMove for Piece {
    fn can_move_nforwards(&self, n: u32, board: &Board) -> ZeroResult<()> {
        let to;

        if self.color.is_white() {
            to = self.meta.rank + n;
        } else {
            to = checked_sub_move(&self, n)?;
        }

        beyond_limits_error(to, 7)?;

        let index = to_index(to, self.meta.file);
        let at_index = &board.0[index as usize];

        if self.piece_type.is_pawn() && !at_index.piece_type.is_empty() {
            return Err(ZeroErr::Illegal {
                at: format!("{}", self.as_rep()),
                why: "The Pawn is blocked and can not jump over pieces nor capture".to_string(),
            });
        }

        same_color_error(&self, at_index)?;
        have_capture_error(&self, at_index)?;

        if self.piece_type.is_pawn() && self.meta.has_moved && n == 2 {
            return Err(ZeroErr::Illegal {
                at: format!("{}", self.as_rep()),
                why: "The Pawn has moved before so it can not move twice now".to_string(),
            });
        }

        if n > 2 {
            return Err(ZeroErr::Illegal {
                at: format!("{}", self.as_rep()),
                why: "A Pawn can not move more than 2 ranks at once".to_string(),
            });
        }

        otherthan!(self ~ PieceType::King | PieceType::Pawn | PieceType::Queen | PieceType::Rook >
            format!("you can not move '{}' forward!", self.piece_type));

        Ok(())
    }

    fn can_move_nleft(&self, n: u32, board: &Board) -> ZeroResult<()> {
        let to = checked_sub_move(&self, n)?;

        let index = to_index(self.meta.rank, to);
        let at_index = board.0[index as usize];

        same_color_error(&self, &at_index)?;
        have_capture_error(&self, &at_index)?;

        otherthan!(self ~ PieceType::Queen | PieceType::Rook | PieceType::King >
            format!("you can not move '{}' to the left!", self.piece_type));

        Ok(())
    }

    fn can_move_nright(&self, n: u32, board: &Board) -> ZeroResult<()> {
        let to = self.meta.file + n;

        beyond_limits_error(to, 7)?;

        let index = to_index(self.meta.rank, to);
        let at_index = board.0[index as usize];

        same_color_error(&self, &at_index)?;
        have_capture_error(&self, &at_index)?;

        otherthan!(self ~ PieceType::Queen | PieceType::Rook | PieceType::King >
            format!("you can not move '{}' to the right!", self.piece_type));

        Ok(())
    }

    fn can_move_nback(&self, n: u32, board: &Board) -> ZeroResult<()> {
        let to;

        if self.color.is_white() {
            to = checked_sub_move(&self, n)?;
        } else {
            to = self.meta.rank + n;
        }

        beyond_limits_error(to, 7)?;

        let at_index = board.0[to_index(to, self.meta.file) as usize];

        same_color_error(&self, &at_index)?;
        have_capture_error(&self, &at_index)?;

        match self.piece_type {
            PieceType::King | PieceType::Queen | PieceType::Rook => {}
            _ => {
                return Err(ZeroErr::Illegal {
                    at: format!("{}", self.as_rep()),
                    why: format!("you can not move '{}' to the back!", self.piece_type),
                });
            }
        }

        otherthan!(self ~ PieceType::King | PieceType::Queen | PieceType::Rook >
            format!("you can not move '{}' to the back!", self.piece_type));

        Ok(())
    }
}

impl Engine {
    pub fn can_move(&mut self, from: u32) -> ZeroResult<()> {
        self.check_turn(from)?;

        Ok(())
    }

    fn check_turn(&self, from: u32) -> ZeroResult<()> {
        if self.turn && self.board.0[from as usize].color.is_black() {
            return Err(ZeroErr::Illegal {
                at: format!("({})", from),
                why: "it's white's turn to move!".to_string(),
            });
        }

        if !self.turn && self.board.0[from as usize].color.is_white() {
            return Err(ZeroErr::Illegal {
                at: format!("({})", from),
                why: "it's black's turn to move!".to_string(),
            });
        }

        Ok(())
    }
}
