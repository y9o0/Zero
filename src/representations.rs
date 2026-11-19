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
