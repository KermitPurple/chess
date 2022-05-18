pub mod pieces;
use pieces::*;

fn main() {
    let p = Piece {
        team: PieceTeam::Black,
        typ: PieceType::Pawn,
    };
    println!("{p}");
}
