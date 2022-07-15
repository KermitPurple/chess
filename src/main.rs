pub mod pieces;
use pieces::*;

type Position = (usize, usize);

#[derive(Copy, Clone)]
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

#[derive(Default, Copy, Clone)]
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
        /// Given a list of relative positions check if the move to be checked is one of those positions
        macro_rules! rel_posns {
            ($list:expr) => {{
                ($list)
                    .iter()
                    .any(|&x| x == (a.0.abs_diff(b.0), a.1.abs_diff(b.1)))
            }};
        }
        macro_rules! check {
            (Pawn_y: $p:expr) => {
                if $p.color == Color::Black {
                    a.1 + 1 == b.1
                } else {
                    a.1.checked_sub(1).map(|x| x == b.1).unwrap_or(false)
                }
            };
            (Rook) => {
                (a.0 == b.0 || a.1 == b.1) && todo!("Ensure no collisions")
            };
            (Bishop) => {
                a.0.abs_diff(b.0) == a.1.abs_diff(b.1) && todo!("Ensure no collisions")
            };
            (Queen) => {
                check!(Rook) || check!(Bishop)
            };
        }
        if [a.0, a.1, b.0, b.1].into_iter().any(|x| x >= 8) || a == b {
            return false;
        }
        if let Some(p1) = self.board[a.1][a.0] {
            if let Some(p2) = self.board[b.1][b.0] {
                if p1.color == p2.color {
                    return false;
                }
                if p1.typ == PieceType::Pawn {
                    if a.0.abs_diff(b.0) == 1 && check!(Pawn_y: p1) {
                        return true;
                    }
                }
            }
            match p1.typ {
                PieceType::Pawn => a.0 == b.0 && check!(Pawn_y: p1) && todo!("The possant thing"),
                PieceType::Rook => check!(Rook),
                PieceType::Knight => rel_posns!([(1, 2), (2, 1)]),
                PieceType::Bishop => check!(Bishop),
                PieceType::Queen => check!(Queen),
                PieceType::King => rel_posns!([(1, 1), (1, 0), (0, 1)]),
            }
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pawn_move_test() {
        todo!();
    }

    #[test]
    fn rook_move_test() {
        todo!();
    }
 
    #[test]
    fn knight_move_test() {
        let b = Board::new();
        assert!(b.valid_move(
            (1, 7), // Black left knight
            (2, 5), // Up two right one
        ));
        assert!(b.valid_move(
            (1, 7), // Black left knight
            (0, 5), // Up two left one
        ));
        assert!(b.valid_move(
            (6, 7), // Black right knight
            (7, 5), // Up two right one
        ));
        assert!(b.valid_move(
            (6, 7), // Black right knight
            (5, 5), // Up two left one
        ));
        assert!(b.valid_move(
            (1, 0), // White left knight
            (2, 2), // Down two right one
        ));
        assert!(b.valid_move(
            (1, 0), // White left knight
            (0, 2), // Down two left one
        ));
        assert!(b.valid_move(
            (6, 0), // White right knight
            (7, 2), // Down two right one
        ));
        assert!(b.valid_move(
            (6, 0), // White right knight
            (5, 2), // Down two left one
        ));
        assert!(!b.valid_move(
            (1, 0), // White right knight
            (3, 0), // Down two left one
        ));
    }
 
    #[test]
    fn bishop_move_test() {
        todo!();
    }
 
    #[test]
    fn queen_move_test() {
        todo!();
    }

    #[test]
    fn king_move_test() {
        todo!();
    }
}

fn main() {
}
