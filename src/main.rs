use crate::{
    engine::core::{Engine, MoveType},
    errors::ZeroResult,
    representations::MoveRep,
};

mod engine;
mod errors;

mod representations;

fn main() -> ZeroResult<()> {
    let mut eng = Engine::new();

    eng.make_move(MoveRep::E(1), MoveType::Forward(2))?;
    eng.make_move(MoveRep::E(6), MoveType::Forward(2))?;
    eng.make_move(MoveRep::D(1), MoveType::Forward(2))?;
    eng.make_move(MoveRep::D(6), MoveType::Forward(1))?;
    eng.make_move(MoveRep::D(0), MoveType::Forward(2))?;
    eng.make_move(MoveRep::A(6), MoveType::Forward(1))?;
    eng.make_move(MoveRep::D(2), MoveType::Left(3))?;
    eng.make_move(MoveRep::A(5), MoveType::Forward(1))?;
    eng.make_move(MoveRep::A(2), MoveType::Right(7))?;

    eng.board.print_board();

    Ok(())
}
