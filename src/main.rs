use crate::{engine::core::Engine, errors::ZeroResult};

mod engine;
mod errors;

mod representations;

fn main() -> ZeroResult<()> {
    let mut eng = Engine::new();

    eng.make_move(12, 2)?;
    eng.make_move(52, 1)?;
    eng.make_move(28, 1)?;

    eng.board.print_board();

    Ok(())
}
