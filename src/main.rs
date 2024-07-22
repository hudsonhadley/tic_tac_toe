use tic_tac_toe::{
    Board,
    play_game
};

fn main() {
    let mut board = Board::new(3);

    match play_game(&mut board) {
        Some(winner) => println!("{} wins!", winner),
        None => println!("Tie!"),
    };
}
