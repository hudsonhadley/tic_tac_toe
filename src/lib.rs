use std::io;
use rand::{
    seq::SliceRandom
};

pub fn play_game(board: &mut Board) -> Option<char> {

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
                println!("Please enter a number 1 - {}", board.size * board.size);
                continue;
            },
        };

        if player_move < 1 || player_move > (board.size * board.size) as i32 {
            println!("Please enter a number 1 - {}", board.size * board.size);
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

pub struct Board {
    spots: Vec<char>,
    size: usize,
}

impl Board {
    pub fn new(size: usize) -> Board {
        Board {
            spots: vec![' '; size * size],
            size,
        }
    }

    fn play(&mut self, spot: i32, player: &char) -> Result<(), String> {
        if *player != 'X' && *player != 'O' {
            panic!("Player must be 'X' or 'O', got {player}.");
        } else if spot < 0 || spot as usize > self.size * self.size - 1 {
            return Err(String::from("Spot must be between 0 and 8, got {spot}"));
        } else if self.spots[spot as usize] != ' ' {
            return Err(String::from("Spot already filled"));
        }

        self.spots[spot as usize] = *player;

        Ok(())
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();

        for i in 0..self.size {
            for j in 0..self.size - 1 {
                output.push(self.spots[i * self.size + j]);
                output.push('|');
            }
            output.push(self.spots[i * self.size + self.size - 1]);

            if i < self.size - 1 {
                output.push('\n');
                for _k in 0..self.size * 2 - 1 { output.push('-'); }
                output.push('\n');
            }
        }

        output
    }

    fn has_won(&self) -> Option<char> {
        // Check rows
        for r in 0..self.size {
            // Get what the first character is
            let first_spot_char = &self.spots[r * self.size];

            // We can pass on the row if the first spot isn't interesting
            if *first_spot_char == ' '{
                continue;
            }

            for c in 1..self.size {
                if self.spots[r * self.size + c] != *first_spot_char {
                    break; // Break out of the row, and move onto the next row
                } else if c == self.size - 1 { // If we get to the end of the row, then they win
                    return Some(*first_spot_char);
                }
            }
        }

        // Check columns
        for c in 0..self.size {
            // Get what the first character is
            let first_spot_char = &self.spots[c];

            // We can pass on the column if the first spot isn't interesting
            if *first_spot_char == ' '{
                continue;
            }

            for r in 1..self.size {
                if self.spots[r * self.size + c] != *first_spot_char {
                    break;
                } else if r == self.size - 1 {
                    return Some(*first_spot_char)
                }
            }
        }

        // Check diagonals
        let first_spot_char = &self.spots[0];

        // We only want to check if the first spot is interesting
        if *first_spot_char != ' ' {
            for i in 1..self.size {
                if self.spots[i * self.size + i] != *first_spot_char {
                    break;
                } else if i == self.size - 1 {
                    return Some(*first_spot_char);
                }
            }
        }

        let first_spot_char = &self.spots[self.size - 1];

        if *first_spot_char != ' ' {
            for i in 2..(self.size + 1) {
                if self.spots[i * self.size - i] != *first_spot_char {
                    break;
                } else if i == self.size {
                    return Some(*first_spot_char);
                }
            }
        }

        None
    }

    fn is_full(&self) -> bool {
        for i in 0..(self.size * self.size - 1) {
            // If we hit a non-blank spot, it isn't full
            if self.spots[i] == ' ' {
                return false
            }
        }

        true
    }


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_win() {
        let mut board = Board::new(3);
        /*
        OXO
        XXO
        OOX
         */
        board.play(0, &'O').expect("Failed to play");
        board.play(1, &'X').expect("Failed to play");
        board.play(2, &'O').expect("Failed to play");
        board.play(3, &'X').expect("Failed to play");
        board.play(4, &'X').expect("Failed to play");
        board.play(5, &'O').expect("Failed to play");
        board.play(6, &'O').expect("Failed to play");
        board.play(7, &'O').expect("Failed to play");
        board.play(8, &'X').expect("Failed to play");

        assert_eq!(None, board.has_won());
    }

    #[test]
    fn test_row_win() {
        let mut board = Board::new(3);

        for i in 3..6 {
            board.play(i, &'X').expect("Failed to play");
        }

        assert_eq!(Some('X'), board.has_won());
    }

    #[test]
    fn test_col_win() {
        let mut board = Board::new(4);

        for i in 0..4 {
            board.play(i * 4, &'O').expect("Failed to play");
        }

        assert_eq!(Some('O'), board.has_won());
    }

    #[test]
    fn test_diagonal_win() {
        let mut board = Board::new(5);

        for i in 0..5 {
            board.play(i * 5 + i, &'X').expect("Failed to play");
        }

        assert_eq!(Some('X'), board.has_won());
    }

    #[test]
    fn test_full_board() {
        let mut board = Board::new(3);

        for i in 0..9 {
            board.play(i, &'X').expect("Failed to play");
        }

        assert!(board.is_full());
    }

    #[test]
    fn test_empty_board() {
        let board = Board::new(3);
        assert!(!board.is_full());
    }
}