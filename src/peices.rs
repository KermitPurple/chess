enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King
}

enum PieceTeam {
    Black,
    White
}

pub struct Piece {
    pub team: PieceTeam,
    pub typ: PieceType
}
