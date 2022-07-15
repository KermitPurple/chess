pub mod pieces;
use pieces::*;

struct CastleRights {
    white_left: bool,
    white_right: bool,
    black_left: bool,
    black_right: bool,
}

impl Default for CastleRights {
    fn default() -> Self {
        Self {
            white_left: true,
            white_right: true,
            black_left: true,
            black_right: true,
        }
    }
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

    fn valid_move(&self, a: (usize, usize), b: (usize, usize)) -> bool {
        if [a.0, a.1, b.0, b.1].into_iter().any(|x| x >= 8) {
            return false;
        }
        match (self.board[a.1][a.0], self.board[b.1][b.0]) {
            (Some(p1), Some(p2)) => {
                if p1.color == p2.color {
                    return false;
                }
                todo!()
            }
            (Some(p), None) => {
                match p.typ {
                    PieceType::Pawn => {
                        let correct_y = if p.color == Color::Black {
                            a.1 + 1 == b.1
                        } else {
                            a.1
                                .checked_sub(1)
                                .map(|x| x == b.1)
                                .unwrap_or(false)
                        };
                        a.0 == b.0 && correct_y
                    }
                    PieceType::Rook => { 
                        (a.0 == b.0 || a.1 == b.1) && todo!("Ensure no collisions")
                    }
                    PieceType::Knight => {
                        for (x, y) in [(1, 2), (2, 1)] {
                            if (a.0 + x == b.0 
                                && a.1 + y == b.1)
                                || (a.0.checked_sub(x)
                                    .map(|n| n == b.0)
                                    .unwrap_or(false)
                                && a.1.checked_sub(y)
                                    .map(|n| n == b.1)
                                    .unwrap_or(false))
                            {
                                return true;
                            }
                        }
                        false
                    }
                    PieceType::Bishop => { todo!() }
                    PieceType::Queen => { todo!() }
                    PieceType::King => { todo!() }
                }
            }
            _ => false
        }
    }
}

fn main() {
    let board = Board::new();
}
