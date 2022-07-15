pub mod pieces;
use pieces::*;

type Position = (usize, usize);

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
    passant_killable: Option<Position>,
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

    fn valid_move(&self, a: Position, b: Position) -> bool {
        macro_rules! rel_posns {
            ($list:expr) => {{
                for (x, y) in $list {
                    if (a.0 + x == b.0 && a.1 + y == b.1)
                        || (a.0.checked_sub(x).map(|n| n == b.0).unwrap_or(false)
                            && a.1.checked_sub(y).map(|n| n == b.1).unwrap_or(false))
                    {
                        return true;
                    }
                }
                false
            }};
        }
        macro_rules! check {
            (Rook) => {
                (a.0 == b.0 || a.1 == b.1) && todo!("Ensure no collisions")
            };
            (Bishop) => {{
                let diff = (
                    if a.0 > b.0 { a.0 - b.0 } else { b.0 - a.0 },
                    if a.1 > b.1 { a.1 - b.1 } else { b.1 - a.1 },
                );
                diff.0 == diff.1 && todo!("Ensure no collisions")
            }};
            (Queen) => {
                check!(Rook) && check!(Bishop)
            };
        }
        if [a.0, a.1, b.0, b.1].into_iter().any(|x| x >= 8) {
            return false;
        }
        match (self.board[a.1][a.0], self.board[b.1][b.0]) {
            (Some(p1), Some(p2)) => {
                if p1.color == p2.color {
                    return false;
                }
                todo!("Capturing peices")
            }
            (Some(p), None) => match p.typ {
                PieceType::Pawn => {
                    let correct_y = if p.color == Color::Black {
                        a.1 + 1 == b.1
                    } else {
                        a.1.checked_sub(1).map(|x| x == b.1).unwrap_or(false)
                    };
                    a.0 == b.0 && correct_y
                }
                PieceType::Rook => check!(Rook),
                PieceType::Knight => rel_posns!([(1, 2), (2, 1)]),
                PieceType::Bishop => check!(Bishop),
                PieceType::Queen => check!(Queen),
                PieceType::King => rel_posns!([(1, 1), (1, 0), (0, 1)]),
            },
            _ => false,
        }
    }
}

fn main() {
    let board = Board::new();
}
