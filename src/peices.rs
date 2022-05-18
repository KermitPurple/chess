enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

impl ToString for PieceType {
    fn to_string(&self) -> String {
        use Self::*;
        match(self) {
            Pawn => "p",
            Rook => "R",
            Knight => "K",
            Bishop => "B",
            Queen => "Q",
            King => "K"
        }
    }
}

enum PieceTeam {
    Black,
    White
}

pub struct Piece {
    pub team: PieceTeam,
    pub typ: PieceType
}
