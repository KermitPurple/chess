pub mod pieces;
use pieces::*;

#[derive(Default)]
struct CastleRights {
    white_left: bool,
    white_right: bool,
    black_left: bool,
    black_right: bool,
}

#[derive(Default)]
struct Board {
    board: [[Option<Piece>; 8]; 8],
    passant_killable: Option<(u8, u8)>,
    castle_rights: CastleRights,
}

impl Board {
    fn new() -> Self {
        Self {
            board: [
                [
                    Some(Piece::new(Color::White, PieceType::Rook)),
                    Some(Piece::new(Color::White, PieceType::Knight)),
                    Some(Piece::new(Color::White, PieceType::Bishop)),
                    Some(Piece::new(Color::White, PieceType::Queen)),
                    Some(Piece::new(Color::White, PieceType::King)),
                    Some(Piece::new(Color::White, PieceType::Bishop)),
                    Some(Piece::new(Color::White, PieceType::Knight)),
                    Some(Piece::new(Color::White, PieceType::Rook)),
                ],
                [Some(Piece::new(Color::White, PieceType::Pawn)); 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [Some(Piece::new(Color::Black, PieceType::Pawn)); 8],
                [
                    Some(Piece::new(Color::Black, PieceType::Rook)),
                    Some(Piece::new(Color::Black, PieceType::Knight)),
                    Some(Piece::new(Color::Black, PieceType::Bishop)),
                    Some(Piece::new(Color::Black, PieceType::Queen)),
                    Some(Piece::new(Color::Black, PieceType::King)),
                    Some(Piece::new(Color::Black, PieceType::Bishop)),
                    Some(Piece::new(Color::Black, PieceType::Knight)),
                    Some(Piece::new(Color::Black, PieceType::Rook)),
                ],
            ],
            ..Default::default()
        }
    }
}

fn main() {
    let board = Board::new();
}
