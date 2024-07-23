use std::io;
use board;
use rand::seq::SliceRandom;

pub fn play_game(board: &mut board::Board) -> Option<char> {

    let all_players = vec!['X', 'O'];
    let mut player = all_players.choose(&mut rand::thread_rng()).unwrap();

    loop {
        println!("{}", board.to_string());
        println!("Enter move {}: ", player);
        let mut player_move = String::from("");

        io::stdin()
            .read_line(&mut player_move)
            .expect("Failed to read line");

        let player_move: i32 = match player_move.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Please enter a number 1 - {}", board.size() * board.size());
                continue;
            },
        };

        if player_move < 1 || player_move > (board.size() * board.size()) as i32 {
            println!("Please enter a number 1 - {}", board.size() * board.size());
            continue;
        } else if let Err(_) = board.play(player_move - 1, player) {
            println!("Please enter an empty spot");
        }


        // See if there is a winner
        match board.has_won() {
            Some(player) => { println!("{}", board.to_string()); return Some(player); },
            None => (),
        }

        // If there isn't a winner, see if it is a tie
        if board.is_full() {
            println!("{}", board.to_string());
            return None
        }

        // Change the player
        if *player == 'X' {
            player = &'O';
        } else {
            player = &'X';
        }
    }
}