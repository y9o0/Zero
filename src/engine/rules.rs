use super::*;

pub trait CanMove {
    fn can_move_nforwards(&self, n: u32, board: &Board) -> ZeroResult<()>;
    fn can_move_nleft(&self, n: u32, board: &Board) -> ZeroResult<()>;
    fn can_move_nright(&self, n: u32, board: &Board) -> ZeroResult<()>;
}

impl CanMove for Piece {
    fn can_move_nforwards(&self, n: u32, board: &Board) -> ZeroResult<()> {
        let to;

        if self.color.is_white() {
            to = self.meta.rank + n;
        } else {
            to = self.meta.rank - n;
        }

        let index = to_index(to, self.meta.file);
        let at_index = &board.0[index as usize];

        match self.piece_type {
            PieceType::Pawn => {
                if self.meta.has_moved && n == 2 {
                    return Err(ZeroErr::Illegal {
                        at: format!("(file : {}, rank : {})", self.meta.file, self.meta.rank),
                        why: "The Pawn has moved before so it can not move twice now".to_string(),
                    });
                }

                if n > 2 {
                    return Err(ZeroErr::Illegal {
                        at: format!("(file : {}, rank : {})", self.meta.file, self.meta.rank),
                        why: "A Pawn can not move more than 2 ranks at once".to_string(),
                    });
                }

                if !at_index.piece_type.is_empty() {
                    return Err(ZeroErr::Illegal {
                        at: format!("(file : {}, rank : {})", self.meta.file, self.meta.rank),
                        why: "The Pawn is blocked and can not jump over pieces".to_string(),
                    });
                }
            }
            PieceType::King => {
                if self.color == at_index.color {
                    return Err(ZeroErr::Illegal {
                        at: format!("(file : {}, rank : {})", self.meta.file, self.meta.rank),
                        why: "The King is blocked by it's own pieces".to_string(),
                    });
                }
            }
            PieceType::Rook => {
                if self.color == at_index.color {
                    return Err(ZeroErr::Illegal {
                        at: format!("(file : {}, rank : {})", self.meta.file, self.meta.rank),
                        why: "The Rook is blocked by it's own pieces".to_string(),
                    });
                }
            }
            PieceType::Queen => {
                if self.color == at_index.color {
                    return Err(ZeroErr::Illegal {
                        at: format!("(file : {}, rank : {})", self.meta.file, self.meta.rank),
                        why: "The Queen is blocked by it's own pieces".to_string(),
                    });
                }
            }
            _ => {
                return Err(ZeroErr::Unimplemented {
                    message: format!("{:?} for MoveForward", self),
                });
            }
        }

        Ok(())
    }

    fn can_move_nleft(&self, n: u32, board: &Board) -> ZeroResult<()> {
        let to = self
            .meta
            .file
            .checked_sub(n)
            .ok_or_else(|| ZeroErr::Unexpected {
                message: "You have went so far beyond the limits of the board!".to_string(),
            })?;

        let index = to_index(self.meta.rank, to);
        let at_index = board.0[index as usize];

        if self.color == at_index.color {
            return Err(ZeroErr::Illegal {
                at: format!("(file : {}, rank : {})", self.meta.file, self.meta.rank),
                why: format!(
                    "the '{}' is blooked by '{}' from the same color",
                    self.piece_type, at_index.piece_type
                ),
            });
        }

        if !at_index.piece_type.is_empty() && self.color != at_index.color {
            return Err(ZeroErr::Illegal {
                at: format!("(file : {}, rank : {})", self.meta.file, self.meta.rank),
                why: "you have to capture the piece, you can not move over it".to_string(),
            });
        }

        match self.piece_type {
            PieceType::Queen | PieceType::Rook | PieceType::King => {}
            _ => {
                return Err(ZeroErr::Illegal {
                    at: format!("(file : {}, rank : {})", self.meta.file, self.meta.rank),
                    why: format!("you can not move '{}' to the left!", self.piece_type),
                });
            }
        }

        Ok(())
    }

    fn can_move_nright(&self, n: u32, board: &Board) -> ZeroResult<()> {
        let to = self.meta.file + n;

        if to > 7 {
            return Err(ZeroErr::Unexpected {
                message: "You have went so far beyond the limits of the board!".to_string(),
            });
        }

        let index = to_index(self.meta.rank, to);
        let at_index = board.0[index as usize];

        if self.color == at_index.color {
            return Err(ZeroErr::Illegal {
                at: format!("(file : {}, rank : {})", self.meta.file, self.meta.rank),
                why: format!(
                    "the '{}' is blooked by '{}' from the same color",
                    self.piece_type, at_index.piece_type
                ),
            });
        }

        if !at_index.piece_type.is_empty() && self.color != at_index.color {
            return Err(ZeroErr::Illegal {
                at: format!("(file : {}, rank : {})", self.meta.file, self.meta.rank),
                why: "you have to capture the piece, you can not move over it".to_string(),
            });
        }

        match self.piece_type {
            PieceType::Queen | PieceType::Rook | PieceType::King => {}
            _ => {
                return Err(ZeroErr::Illegal {
                    at: format!("(file : {}, rank : {})", self.meta.file, self.meta.rank),
                    why: format!("you can not move '{}' to the left!", self.piece_type),
                });
            }
        }

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
