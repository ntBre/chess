use chess::board::Board;

fn main() {
    let board = Board::starting_position();
    println!("{}", board);
    println!("initial eval= {:.2}", board.evaluate());
}
