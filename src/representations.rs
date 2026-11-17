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

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bishop => write!(f, "bishop"),
            Self::King => write!(f, "king"),
            Self::None => write!(f, "none"),
            Self::Pawn => write!(f, "pawn"),
            Self::Queen => write!(f, "queen"),
            Self::Rook => write!(f, "rook"),
            Self::Knight => write!(f, "knight"),
        }
    }
}

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

#[derive(Debug, Clone, Copy)]
pub enum Color {
    White,
    Black,

    None,
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Default for Piece {
    fn default() -> Self {
        Self {
            piece_type: PieceType::None,
            color: Color::None,
        }
    }
}

fn empty() -> Piece {
    Piece {
        piece_type: PieceType::None,
        color: Color::None,
    }
}

fn add(color: Color, piece: PieceType) -> Piece {
    Piece {
        piece_type: piece,
        color,
    }
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
}
