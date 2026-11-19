use crate::engine::rules::CanMoveForward;

use super::*;

pub trait MoveForward {
    fn move_nforward(&mut self, board: &mut Board, n: u32) -> ZeroResult<()>;
}

impl MoveForward for Piece {
    fn move_nforward(&mut self, board: &mut Board, n: u32) -> ZeroResult<()> {
        self.can_move_nforwards(n, board)?;

        let base_index = to_index(self.meta.rank, self.meta.file);

        if self.color.is_white() {
            self.meta.rank += n;
        } else {
            self.meta.rank -= n;
        }

        if self.piece_type.is_pawn() {
            self.meta.has_moved = true;
        }

        let index = to_index(self.meta.rank, self.meta.file);
        let tooken = std::mem::take(&mut board.0[base_index as usize]);
        board.0[index as usize] = tooken;

        Ok(())
    }
}
