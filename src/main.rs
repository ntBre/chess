use board::Board;

mod board;
mod piece;
mod square;

fn main() {
    let board = Board::starting_position();
    println!("{}", board);
}
