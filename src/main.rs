use crate::{
    engine::core::{
        Engine,
        MoveType::{self, Back, Forward, Right},
    },
    errors::ZeroResult,
    representations::MoveRep,
};

mod engine;
mod errors;

mod representations;

fn main() -> ZeroResult<()> {
    let mut eng = Engine::new();

    eng.make_move(MoveRep::D(1), Forward(2))?;
    eng.make_move(MoveRep::D(6), Forward(2))?;
    eng.make_move(MoveRep::D(0), Forward(2))?;
    eng.make_move(MoveRep::D(7), Forward(2))?;
    eng.make_move(MoveRep::D(2), Right(2))?;
    eng.make_move(MoveRep::D(5), Right(2))?;
    eng.make_move(MoveRep::F(2), MoveType::Left(2))?;
    eng.make_move(MoveRep::F(5), MoveType::Left(2))?;
    eng.make_move(MoveRep::D(2), Back(2))?;
    eng.make_move(MoveRep::D(5), Back(2))?;

    eng.board.print_board();

    Ok(())
}
