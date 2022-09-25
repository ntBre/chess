use board::Board;

mod board;
mod coord;
mod piece;
mod square;

#[cfg(test)]
mod tests;

fn main() {
    let board = Board::starting_position();
    println!("{}", board);
    println!("initial eval= {:.2}", board.evaluate());
}
