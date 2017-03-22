use std::io::prelude::*;
use std::str;
use std::io;
use std::option::Option;
use std::fmt::Display;
use std::fmt;


#[derive(Debug, Copy, Clone, PartialEq)]
enum Player {
    X,
    O
}

type Board = [Option<Player>; 9];


#[derive(Debug)]
struct Game {
    current_player: Player,
    board: Board,
}


impl Game {

    fn format_player(&self, player: Option<Player>, f: &mut fmt::Formatter) {
        match player {
            None => write!(f, " "),
            Some(Player::X) => write!(f, "X"),
            Some(Player::O) => write!(f, "O")
        }.ok();
    }

    fn format_row(&self, idx: usize, f: &mut fmt::Formatter) {
        self.format_player(self.board[idx + 0], f);
        write!(f, " | ").ok();
        self.format_player(self.board[idx + 1], f);
        write!(f, " | ").ok();
        self.format_player(self.board[idx + 2], f);
    }

    fn format_board(&self, f: &mut fmt::Formatter) {
        for idx in 0..3 {
            self.format_row(idx * 3, f);
            if idx != 2 {
                write!(f, "\n----------\n").ok();
            }
        }
    }

    fn do_move(&mut self, idx: usize) {
        if self.board[idx].is_some() {
            println!("Not a valid move!");
            return;
        }

        self.board[idx] = Some(self.current_player);
        self.current_player = match self.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }

    fn print_status(&self) {
        println!("{}", self);
        println!("\n\n");
        println!("Current Turn: {:?}", self.current_player)
    }

    fn check_condition(&self, cond: [usize; 3]) -> Option<Player> {
        if self.board[cond[0]] == self.board[cond[1]] && self.board[cond[0]] == self.board[cond[2]] {
            self.board[cond[0]]
        } else {
            None
        }
    }

    fn check_win_condition(&self) -> Option<Option<Player>> {
        let conditions = [
            [0, 1, 2], [3, 4, 5], [6, 7, 8],
            [0, 3, 6], [1, 4, 7], [2, 5, 8],
            [0, 4, 8], [2, 4, 6]];

        let mut wins = conditions
            .iter()
            .cloned()
            .map(|c| self.check_condition(c))
            .filter(|p| p.is_some())
            .collect::<Vec<Option<Player>>>();

        let is_full = self.board.iter().all(|p| p.is_some());
        if is_full {
            wins.push(None);
        }

        wins.into_iter().nth(0)
    }


    fn process_turn(&mut self, input: &str) {

        println!();
        let c = input.chars().nth(0).and_then(|n| n.to_digit(10));

        match c {
            Some(n) if n > 0 => self.do_move((n - 1) as usize),
            _ => println!("'{}' is not a valid move!\n", input)
        }

        println!("{}\n", self);

    }

    fn run(&mut self) {
        let stdin = io::stdin();
        println!("Turn {:?}", self.current_player);
        for line in stdin.lock().lines().map(|line| line.unwrap()) {
            self.process_turn(&line);

            match self.check_win_condition() {
                Some(Some(p)) => {println!("{:?} is the winner!", p); return},
                Some(None) => {println!("Cat's game!"); return},
                None => continue
            }
        }

    }
}


impl Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.format_board(f);
        write!(f, "")
    }

}


fn main() {
    loop {
        let mut game = Game{
            current_player: Player::X,
            board: [None; 9],
        };

        println!("New Game!");
        game.print_status();
        game.run();
    }
}
