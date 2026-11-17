use crate::{engine::Engine, errors::ZeroResult};

mod engine;
mod errors;
mod representations;

fn main() -> ZeroResult<()> {
    let mut eng = Engine::new();

    eng.make_move(12, 28)?;

    eng.undo();

    Ok(())
}
