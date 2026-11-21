use std::fmt;

#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Rook,
    King,
    Knight,
    Queen,
    Bishop,

    None,
}

#[allow(warnings)]
pub enum MoveRep {
    A(u32),
    B(u32),
    C(u32),
    D(u32),
    E(u32),
    F(u32),
    G(u32),
    H(u32),
}

impl MoveRep {
    pub fn to_index(&self) -> u32 {
        match self {
            Self::A(n) => to_index(*n, 0),
            Self::B(n) => to_index(*n, 1),
            Self::C(n) => to_index(*n, 2),
            Self::D(n) => to_index(*n, 3),
            Self::E(n) => to_index(*n, 4),
            Self::F(n) => to_index(*n, 5),
            Self::G(n) => to_index(*n, 6),
            Self::H(n) => to_index(*n, 7),
        }
    }
}

impl fmt::Display for MoveRep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::A(x) => write!(f, "a{}", x),
            Self::B(x) => write!(f, "b{}", x),
            Self::D(x) => write!(f, "d{}", x),
            Self::E(x) => write!(f, "e{}", x),
            Self::G(x) => write!(f, "g{}", x),
            Self::C(x) => write!(f, "c{}", x),
            Self::F(x) => write!(f, "f{}", x),
            Self::H(x) => write!(f, "h{}", x),
        }
    }
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bishop => write!(f, "bishop"),
            Self::King => write!(f, "king"),
            Self::Knight => write!(f, "knight"),
            Self::None => write!(f, "empty"),
            Self::Pawn => write!(f, "pawn"),
            Self::Queen => write!(f, "queen"),
            Self::Rook => write!(f, "rook"),
        }
    }
}

#[allow(warnings)]
impl PieceType {
    pub fn is_pawn(&self) -> bool {
        match self {
            Self::Pawn => true,
            _ => false,
        }
    }

    pub fn is_knight(&self) -> bool {
        match self {
            Self::Knight => true,
            _ => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::None => true,
            _ => false,
        }
    }
}

impl Color {
    pub fn is_white(&self) -> bool {
        match self {
            Self::White => true,
            _ => false,
        }
    }

    #[allow(warnings)]
    pub fn is_black(&self) -> bool {
        match self {
            Self::Black => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    White,
    Black,

    None,
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
    pub meta: Meta,
}

impl Piece {
    pub fn as_rep(&self) -> MoveRep {
        let r = self.meta.rank;

        match self.meta.file {
            0 => MoveRep::A(r),
            1 => MoveRep::B(r),
            2 => MoveRep::C(r),
            3 => MoveRep::D(r),
            4 => MoveRep::E(r),
            5 => MoveRep::F(r),
            6 => MoveRep::G(r),
            7 => MoveRep::H(r),
            _ => unreachable!(
                "This branch should never be hit, the file number doesn't match '{}' anything!",
                self.meta.file
            ),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Meta {
    pub has_moved: bool,
    pub rank: u32,
    pub file: u32,
}

impl Meta {
    pub fn new() -> Self {
        Self {
            has_moved: false,
            rank: 0,
            file: 0,
        }
    }
}

impl Default for Piece {
    fn default() -> Self {
        Self {
            piece_type: PieceType::None,
            color: Color::None,
            meta: Meta::new(),
        }
    }
}

fn empty() -> Piece {
    Piece {
        piece_type: PieceType::None,
        color: Color::None,
        meta: Meta::new(),
    }
}

fn add(color: Color, piece: PieceType) -> Piece {
    Piece {
        piece_type: piece,
        color,
        meta: Meta::new(),
    }
}

pub trait Ranks {
    fn to_rank(&self) -> u32;
}

pub trait File {
    fn to_file(&self) -> u32;
}

impl Ranks for u32 {
    fn to_rank(&self) -> u32 {
        self / 8
    }
}

impl File for u32 {
    fn to_file(&self) -> u32 {
        self % 8
    }
}

pub fn to_index(rank: u32, file: u32) -> u32 {
    rank * 8 + file
}

pub struct Board(pub [Piece; 64]);

impl Board {
    pub fn new() -> Self {
        Self([
            add(Color::White, PieceType::Rook),
            add(Color::White, PieceType::Knight),
            add(Color::White, PieceType::Bishop),
            add(Color::White, PieceType::Queen),
            add(Color::White, PieceType::King),
            add(Color::White, PieceType::Bishop),
            add(Color::White, PieceType::Knight),
            add(Color::White, PieceType::Rook),
            add(Color::White, PieceType::Pawn),
            add(Color::White, PieceType::Pawn),
            add(Color::White, PieceType::Pawn),
            add(Color::White, PieceType::Pawn),
            add(Color::White, PieceType::Pawn),
            add(Color::White, PieceType::Pawn),
            add(Color::White, PieceType::Pawn),
            add(Color::White, PieceType::Pawn),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            empty(),
            add(Color::Black, PieceType::Pawn),
            add(Color::Black, PieceType::Pawn),
            add(Color::Black, PieceType::Pawn),
            add(Color::Black, PieceType::Pawn),
            add(Color::Black, PieceType::Pawn),
            add(Color::Black, PieceType::Pawn),
            add(Color::Black, PieceType::Pawn),
            add(Color::Black, PieceType::Pawn),
            add(Color::Black, PieceType::Rook),
            add(Color::Black, PieceType::Knight),
            add(Color::Black, PieceType::Bishop),
            add(Color::Black, PieceType::Queen),
            add(Color::Black, PieceType::King),
            add(Color::Black, PieceType::Bishop),
            add(Color::Black, PieceType::Knight),
            add(Color::Black, PieceType::Rook),
        ])
    }

    fn to_pieces_uni(p: &Piece) -> String {
        let res = match (p.piece_type, p.color) {
            (PieceType::King, Color::Black) => "♔",
            (PieceType::Queen, Color::Black) => "♕",
            (PieceType::Rook, Color::Black) => "♖",
            (PieceType::Bishop, Color::Black) => "♗",
            (PieceType::Knight, Color::Black) => "♘",
            (PieceType::Pawn, Color::Black) => "♙",
            (PieceType::King, Color::White) => "♚",
            (PieceType::Queen, Color::White) => "♛",
            (PieceType::Rook, Color::White) => "♜",
            (PieceType::Bishop, Color::White) => "♝",
            (PieceType::Knight, Color::White) => "♞",
            (PieceType::Pawn, Color::White) => "♟",
            (PieceType::None, Color::None) => " ",
            (_, Color::None) => "",
            (PieceType::None, _) => "",
        };

        res.to_string()
    }

    pub fn print_board(&self) {
        let mut index = 0;

        for i in self.0 {
            index += 1;

            let p = Self::to_pieces_uni(&i);
            print!("| {:<4} ", p);
            if index == 8 {
                print!("|");

                println!();

                index = 0;
            }
        }
    }
}
