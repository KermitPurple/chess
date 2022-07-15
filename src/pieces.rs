use std::fmt;

const RESET_COLOR: &str = "\x1b[0m";

#[derive(Copy, Clone)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PieceType::*;
        let s = match self {
            Pawn => "p",
            Rook => "R",
            Knight => "K",
            Bishop => "B",
            Queen => "Q",
            King => "K"
        };
        write!(f, "{}", s)
    }
}

#[derive(Copy, Clone)]
pub enum Color {
    Black,
    White
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Color::*;
        // TODO probably choose better background colors
        let s = match self {
            Black => "\x1b[30;107m",
            White => RESET_COLOR,
        };
        write!(f, "{}", s)
    }
}

#[derive(Copy, Clone)]
pub struct Piece {
    pub color: Color,
    pub typ: PieceType
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.color, self.typ, RESET_COLOR)
    }
}

impl Piece {
    pub fn new(color: Color, typ: PieceType) -> Self {
        Self { color, typ }
    }
}
