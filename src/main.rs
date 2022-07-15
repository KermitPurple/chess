// TODO check_maker and check_freeer
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

    fn test_move(&self, a: Position, b: Position) -> Self {
        let mut board = *self;
        board.make_move(a, b);
        board
    }

    fn make_move(&mut self, a: Position, b: Position) {
        let p = self.board[a.1][a.0].take();
        self.board[b.1][b.0] = p;
    }

    fn in_check(&self, color: Color) -> bool {
        let range = match color {
            Color::Black => 0..=7,
            Color::White => 7..=0,
        };
        let king = 'outer: loop {
            for y in range {
                for x in 0..8 {
                    if matches!(
                        self.board[y][x],
                        Some(Piece { color: c, typ: PieceType::King, .. }) if color == c
                    ) {
                        break 'outer (x, y);
                    }
                }
            }
            panic!("Couln't find king!");
        };
        for y in 0..8 {
            for x in 0..8 {
                if matches!(
                    self.board[y][x],
                    Some(Piece { color: c, ..}) if color != c
                ) && self.valid_move((x, y), king)
                {
                    return true;
                }
            }
        }
        false
    }

    /// Given a list of relative positions check if the move to be checked is one of those positions
    fn rel_posns(self, list: &[Position], a: Position, b: Position) -> bool {
        list.iter()
            .any(|&x| x == (a.0.abs_diff(b.0), a.1.abs_diff(b.1)))
    }

    fn valid_move(&self, a: Position, b: Position) -> bool {
        macro_rules! check {
            (Pawn_y: $p:expr) => {
                if $p.color == Color::Black {
                    a.1 == b.1 + 1
                        // check for two spot jump
                        || (a.1 == 6 && b.1 == 4 && self.board[5][a.0].is_none())
                } else {
                    a.1 + 1 == b.1
                        // check for two spot jump
                        || (a.1 == 1 && b.1 == 3 && self.board[2][a.0].is_none())
                }
            };
            (Rook) => {
                if a.0 == b.0 {
                    let range = if a.1 > b.1 {
                        b.1 + 1..a.1
                    } else {
                        a.1 + 1..b.1
                    };
                    self.board[range]
                        .iter()
                        .all(|x| x[a.0].is_none())
                } else if a.1 == b.1 {
                    let range = if a.0 > b.0 {
                        b.0 + 1..a.0
                    } else {
                        a.0 + 1..b.0
                    };
                    self.board[a.1][range]
                        .iter()
                        .all(|x| x.is_none())
                } else {
                    false
                }
            };
            (Bishop) => {
                if a.0.abs_diff(b.0) == a.1.abs_diff(b.1) {
                    let mut curr = (a.0.min(b.0) + 1, a.1.min(b.1) + 1);
                    let end = (a.0.max(b.0), a.1.max(b.1));
                    loop {
                        if curr >= end {
                            break true;
                        }
                        if self.board[curr.1][curr.0].is_some() {
                            break false;
                        }
                        curr.0 += 1;
                        curr.1 += 1;
                    }
                } else {
                    false
                }
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
                    return a.0.abs_diff(b.0) == 1 && check!(Pawn_y: p1);
                }
            }
            match p1.typ {
                PieceType::Pawn => {
                    (a.0 == b.0 && check!(Pawn_y: p1))
                        || (self
                            .passant_killable
                            .map(|x| x == (b.0, a.1))
                            .unwrap_or(false)
                            && a.1.abs_diff(b.1) == 1)
                }
                PieceType::Rook => check!(Rook),
                PieceType::Knight => self.rel_posns(&[(1, 2), (2, 1)], a, b),
                PieceType::Bishop => check!(Bishop),
                PieceType::Queen => check!(Queen),
                PieceType::King => {
                    if self.rel_posns(&[(1, 1), (1, 0), (0, 1)], a, b) {
                        !self.test_move(a, b).in_check(p1.color)
                    } else {
                        false
                    }
                }
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
        let b = Board::new();
        for x in 0..8 {
            assert!(b.valid_move((x, 1), (x, 2)));
            assert!(b.valid_move((x, 1), (x, 3)));
            assert!(b.valid_move((x, 6), (x, 5)));
            assert!(b.valid_move((x, 6), (x, 4)));
        }
        let b = Board {
            board: [
                [Some(Piece::new(Color::White, PieceType::Pawn)); 8],
                [Some(Piece::new(Color::Black, PieceType::Pawn)); 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
            ],
            ..Default::default()
        };
        for x in 0..8 {
            assert!(!b.valid_move((x, 0), (x, 1))); // move white to black
            assert!(!b.valid_move((x, 1), (x, 0))); // move black to white
        }
        for x in 0..7 {
            assert!(b.valid_move((x, 0), (x + 1, 1))); // move white to black Diagonally
            assert!(b.valid_move((x, 1), (x + 1, 0))); // move black to white Diagonally
            assert!(b.valid_move((x + 1, 0), (x, 1))); // move white to black Diagonally
            assert!(b.valid_move((x + 1, 1), (x, 0))); // move black to white Diagonally
        }
        let b = Board {
            board: [
                [None; 8],
                [
                    Some(Piece::new(Color::White, PieceType::Pawn)),
                    Some(Piece::new(Color::Black, PieceType::Pawn)),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
            ],
            passant_killable: Some((0, 1)),
            ..Default::default()
        };
        assert!(b.valid_move((1, 1), (1, 0)));
        assert!(b.valid_move((1, 1), (0, 0)));
        assert!(!b.valid_move((1, 1), (2, 0)));
    }

    #[test]
    fn rook_move_test() {
        let b = Board::new();
        assert!(!b.valid_move((0, 0), (0, 3))); // Try to move thru piece on same team
        assert!(!b.valid_move((0, 0), (0, 7))); // Try to move thru pieces of multiple colors
        assert!(!b.valid_move((0, 0), (3, 0))); // Try to move thru pieces on same team
        assert!(!b.valid_move((7, 0), (7, 3))); // Try to move thru piece
        assert!(!b.valid_move((7, 0), (7, 7))); // Try to move thru pieces of multiple colors
        assert!(!b.valid_move((7, 0), (3, 0))); // Try to move thru pieces on same team
        let b = Board {
            board: [
                [Some(Piece::new(Color::White, PieceType::Rook)); 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [Some(Piece::new(Color::Black, PieceType::Rook)); 8],
            ],
            ..Default::default()
        };
        // vertical movement
        for x in 0..8 {
            assert!(b.valid_move((x, 0), (x, 7))); // white takes black
            assert!(b.valid_move((x, 0), (x, 4))); // white moves without take
            assert!(b.valid_move((x, 7), (x, 0))); // black takes white
            assert!(b.valid_move((x, 7), (x, 4))); // black moves without take
        }
        let b = Board {
            board: [[
                Some(Piece::new(Color::White, PieceType::Rook)),
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Piece::new(Color::Black, PieceType::Rook)),
            ]; 8],
            ..Default::default()
        };
        // horizontal movement
        for y in 0..8 {
            assert!(b.valid_move((0, y), (7, y))); // white takes black
            assert!(b.valid_move((0, y), (4, y))); // white moves without take
            assert!(b.valid_move((7, y), (0, y))); // black takes white
            assert!(b.valid_move((7, y), (4, y))); // black moves without take
        }
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
        let b = Board {
            board: [
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [
                    None,
                    None,
                    None,
                    None,
                    Some(Piece::new(Color::Black, PieceType::Knight)),
                    None,
                    None,
                    None,
                ],
                [None; 8],
                [None; 8],
                [None; 8],
            ],
            ..Default::default()
        };
        for pos in [
            (5, 6),
            (3, 6),
            (5, 2),
            (3, 2),
            (6, 5),
            (6, 3),
            (2, 5),
            (2, 3),
        ] {
            assert!(b.valid_move(
                (4, 4), // Knight in the center
                pos
            ));
        }
    }

    #[test]
    fn bishop_move_test() {
        let b = Board::new();
        assert!(!b.valid_move((0, 0), (7, 7)));
        assert!(!b.valid_move((0, 0), (4, 4)));
        let b = Board {
            board: [
                [Some(Piece::new(Color::White, PieceType::Bishop)); 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [Some(Piece::new(Color::Black, PieceType::Bishop)); 8],
            ],
            ..Default::default()
        };
        assert!(b.valid_move((0, 0), (7, 7))); // white takes black
        assert!(b.valid_move((7, 0), (0, 7))); // white takes black
        assert!(b.valid_move((7, 7), (0, 0))); // black takes white
        assert!(b.valid_move((0, 7), (7, 0))); // black takes white
        assert!(b.valid_move((0, 0), (4, 4))); // white moves without taking
        assert!(b.valid_move((7, 7), (4, 4))); // black moves without taking
        assert!(b.valid_move((1, 0), (7, 6))); // white moves without taking
        assert!(b.valid_move((1, 0), (4, 3))); // white moves without taking
    }

    #[test]
    fn queen_move_test() {
        let b = Board::new();
        // moving vertically thru peices
        assert!(!b.valid_move((3, 0), (3, 7)));
        assert!(!b.valid_move((3, 7), (3, 0)));
        // moving diagonally thru peices
        assert!(!b.valid_move((3, 0), (7, 4)));
        assert!(!b.valid_move((3, 0), (0, 3)));
        // move horizontally thru peices
        assert!(!b.valid_move((3, 0), (3, 3))); // try to move thru piece on same team
        assert!(!b.valid_move((3, 0), (3, 7))); // try to move thru pieces of multiple colors
        assert!(!b.valid_move((3, 0), (0, 0))); // try to move thru pieces on same team
        let b = Board {
            board: [
                [Some(Piece::new(Color::White, PieceType::Queen)); 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [Some(Piece::new(Color::Black, PieceType::Queen)); 8],
            ],
            ..Default::default()
        };
        // vertical moves
        for x in 0..8 {
            assert!(b.valid_move((x, 0), (x, 7))); // white takes black
            assert!(b.valid_move((x, 0), (x, 4))); // white moves without take
            assert!(b.valid_move((x, 7), (x, 0))); // black takes white
            assert!(b.valid_move((x, 7), (x, 4))); // black moves without take
        }
        // diagonal moves
        assert!(b.valid_move((0, 0), (7, 7))); // white takes black
        assert!(b.valid_move((7, 0), (0, 7))); // white takes black
        assert!(b.valid_move((7, 7), (0, 0))); // black takes white
        assert!(b.valid_move((0, 7), (7, 0))); // black takes white
        assert!(b.valid_move((0, 0), (4, 4))); // white moves without taking
        assert!(b.valid_move((7, 7), (4, 4))); // black moves without taking
        assert!(b.valid_move((1, 0), (7, 6))); // white moves without taking
        assert!(b.valid_move((1, 0), (4, 3))); // white moves without taking
        let b = Board {
            board: [[
                Some(Piece::new(Color::White, PieceType::Queen)),
                None,
                None,
                None,
                None,
                None,
                None,
                Some(Piece::new(Color::Black, PieceType::Queen)),
            ]; 8],
            ..Default::default()
        };
        // Horizontal moves
        for y in 0..8 {
            assert!(b.valid_move((0, y), (7, y))); // white takes black
            assert!(b.valid_move((0, y), (4, y))); // white moves without take
            assert!(b.valid_move((7, y), (0, y))); // black takes white
            assert!(b.valid_move((7, y), (4, y))); // black moves without take
        }
    }

    #[test]
    fn king_move_test() {
        let b = Board::new();
        for pos in [(3, 0), (3, 1), (4, 1), (5, 1), (5, 0)] {
            assert!(!b.valid_move(
                (4, 0), // White King
                pos
            ));
        }
        for pos in [(3, 7), (3, 6), (4, 6), (5, 6), (5, 7)] {
            assert!(!b.valid_move(
                (4, 7), // Black King
                pos
            ));
        }
        let b = Board {
            board: [
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [
                    None,
                    None,
                    None,
                    None,
                    Some(Piece::new(Color::Black, PieceType::King)),
                    None,
                    None,
                    None,
                ],
                [None; 8],
                [None; 8],
                [None; 8],
            ],
            ..Default::default()
        };
        for pos in [
            (4, 5),
            (5, 5),
            (5, 4),
            (5, 3),
            (4, 3),
            (3, 3),
            (3, 4),
            (3, 5),
        ] {
            assert!(b.valid_move(
                (4, 4), // King in center
                pos
            ));
        }
        let b = Board {
            board: [
                [
                    Some(Piece::new(Color::White, PieceType::Bishop)),
                    Some(Piece::new(Color::Black, PieceType::King)),
                    None,
                    None,
                    None,
                    None,
                    None,
                    None,
                ],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
            ],
            ..Default::default()
        };
        assert!(b.valid_move((1, 0), (0, 0)));
        assert!(b.valid_move((1, 0), (0, 1)));
        assert!(!b.valid_move((1, 0), (1, 1)));
        // TODO more extensive tests
    }

    #[test]
    fn in_check_test(){
        todo!();
    }
}

fn main() {}
