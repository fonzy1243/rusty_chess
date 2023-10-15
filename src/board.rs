use crate::piece::Piece;

#[derive(Clone, Copy, Debug)]
pub struct Board {
    pub squares: [i32; 64],
}

impl Board {
    pub fn new() -> Self {
        Self { squares: [0; 64] }
    }

    pub fn read_fen(&mut self, fen: &str) {
        let board = match fen.split_whitespace().next() {
            Some(board) => board,
            None => {
                eprintln!("Failed to split FEN string.");
                return;
            }
        };

        // rank = row, file = column
        let (mut rank, mut file) = (7usize, 0usize);

        for c in board.chars() {
            if c == '/' {
                file = 0;
                rank -= 1;
            } else if c.is_ascii_digit() {
                file += c.to_digit(10).unwrap() as usize;
            } else {
                let color: i32 = if c.is_uppercase() {
                    Piece::White as i32
                } else {
                    Piece::Black as i32
                };

                let piece_type = match c.to_ascii_lowercase() {
                    'p' => 1,
                    'b' => 2,
                    'n' => 3,
                    'r' => 4,
                    'q' => 5,
                    'k' => 6,
                    _ => 0,
                };

                self.squares[rank * 8 + file] = color | piece_type;

                file += 1;
            }
        }
    }
}
