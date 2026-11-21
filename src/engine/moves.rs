use crate::engine::rules::CanMove;

use super::*;

pub trait MoveForward {
    fn move_nforward(&mut self, board: &mut Board, n: u32) -> ZeroResult<()>;
    fn move_nleft(&mut self, board: &mut Board, n: u32) -> ZeroResult<()>;
    fn move_nright(&mut self, board: &mut Board, n: u32) -> ZeroResult<()>;
    fn move_nback(&mut self, board: &mut Board, n: u32) -> ZeroResult<()>;
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

        self.meta.has_moved = true;

        let index = to_index(self.meta.rank, self.meta.file);
        move_place(base_index, index, board);

        Ok(())
    }

    fn move_nleft(&mut self, board: &mut Board, n: u32) -> ZeroResult<()> {
        self.can_move_nleft(n, board)?;

        let base_index = to_index(self.meta.rank, self.meta.file);

        self.meta.file -= n;
        self.meta.has_moved = true;

        let index = to_index(self.meta.rank, self.meta.file);
        move_place(base_index, index, board);

        Ok(())
    }

    fn move_nright(&mut self, board: &mut Board, n: u32) -> ZeroResult<()> {
        self.can_move_nright(n, board)?;

        let base_index = to_index(self.meta.rank, self.meta.file);

        self.meta.file += n;
        self.meta.has_moved = true;

        let index = to_index(self.meta.rank, self.meta.file);
        move_place(base_index, index, board);

        Ok(())
    }

    fn move_nback(&mut self, board: &mut Board, n: u32) -> ZeroResult<()> {
        self.can_move_nback(n, board)?;

        let base_index = to_index(self.meta.rank, self.meta.file);

        if self.color.is_white() {
            self.meta.rank -= n;
        } else {
            self.meta.rank += n;
        }

        self.meta.has_moved = true;

        let index = to_index(self.meta.rank, self.meta.file);
        move_place(base_index, index, board);

        Ok(())
    }
}

fn move_place(from: u32, to: u32, board: &mut Board) {
    let from = std::mem::take(&mut board.0[from as usize]);
    board.0[to as usize] = from
}
