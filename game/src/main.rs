use board;
use game;

fn main() {
    let mut board = board::Board::new(3);

    match game::play_game(&mut board) {
        Some(winner) => println!("{} wins!", winner),
        None => println!("Tie!"),
    };
}
