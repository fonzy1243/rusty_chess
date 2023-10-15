#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub enum Piece {
    // piece type
    None = 0,
    Pawn = 1,
    Bishop = 2,
    Knight = 3,
    Rook = 4,
    Queen = 5,
    King = 6,
    // piece side (01 - white, 10 - black)
    White = 8,
    Black = 16,
}
