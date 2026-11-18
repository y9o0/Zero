use crate::{engine::Engine, errors::ZeroResult};

mod engine;
mod errors;
mod representations;
mod rules;

fn main() -> ZeroResult<()> {
    let mut eng = Engine::new();

    eng.make_move(12, 20)?;
    eng.make_move(52, 44)?;
    eng.make_move(11, 27)?;
    eng.make_move(44, 36)?;

    eng.board.print_board();

    Ok(())
}
