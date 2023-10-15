use board::Board;
use piece::Piece;

mod board;
mod piece;

fn main() {
    let mut board = Board::new();

    board.read_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");

    for rank in (0..8).rev() {
        for file in 0..8 {
            let piece = board.squares[rank * 8 + file];

            let ch = match piece {
                p if p == (Piece::White as i32 | Piece::Pawn as i32) => 'P',
                p if p == (Piece::White as i32 | Piece::Bishop as i32) => 'B',
                p if p == (Piece::White as i32 | Piece::Knight as i32) => 'N',
                p if p == (Piece::White as i32 | Piece::Rook as i32) => 'R',
                p if p == (Piece::White as i32 | Piece::Queen as i32) => 'Q',
                p if p == (Piece::White as i32 | Piece::King as i32) => 'K',
                p if p == (Piece::Black as i32 | Piece::Pawn as i32) => 'p',
                p if p == (Piece::Black as i32 | Piece::Bishop as i32) => 'b',
                p if p == (Piece::Black as i32 | Piece::Knight as i32) => 'n',
                p if p == (Piece::Black as i32 | Piece::Rook as i32) => 'r',
                p if p == (Piece::Black as i32 | Piece::Queen as i32) => 'q',
                p if p == (Piece::Black as i32 | Piece::King as i32) => 'k',
                _ => '.',
            };

            print!("{}", ch);
        }

        println!();
    }
}
