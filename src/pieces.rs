use std::fmt;

const RESET_COLOR: &str = "\x1b[0m";

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

pub enum PieceTeam {
    Black,
    White
}

impl fmt::Display for PieceTeam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use PieceTeam::*;
        // TODO probably choose better background colors
        let s = match self {
            Black => "\x1b[30;107m",
            White => RESET_COLOR,
        };
        write!(f, "{}", s)
    }
}

pub struct Piece {
    pub team: PieceTeam,
    pub typ: PieceType
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}{}", self.team, self.typ, RESET_COLOR)
    }
}
