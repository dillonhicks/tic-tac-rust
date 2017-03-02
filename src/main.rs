#![feature(exclusive_range_pattern)]
use std::io::prelude::*;
use std::str;
use std::io::{self, Read};
use std::option::Option;
use std::fmt::Display;
use std::fmt;


#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
#[derive(PartialEq)]
enum Player {
    None,
    X,
    O
}

type Board = [Player; 9];


#[derive(Debug)]
struct Game {
    current_player: Player,
    board: Board,
}

type Point = [usize; 2];


impl Game {

    fn format_player(&self, player: Player, f: &mut fmt::Formatter) {
        match player {
            Player::None => write!(f, " "),
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O")
        };

    }

    fn format_row(&self, idx: usize, f: &mut fmt::Formatter) {
        self.format_player(self.board[idx + 0], f);
        write!(f, " | ");
        self.format_player(self.board[idx + 1], f);
        write!(f, " | ");
        self.format_player(self.board[idx + 2], f);
    }

    fn format_board(&self, f: &mut fmt::Formatter) {
        for idx in 0..3 {
            self.format_row(idx * 3, f);
            if idx != 2 {
                write!(f, "\n----------\n");
            }
        }
    }

    fn do_move(&mut self, idx: usize) {
        if self.board[idx] != Player::None {
            println!("Not a valid move!");
            return;
        }

        self.board[idx] = self.current_player;
        match self.current_player {
            Player::X => self.current_player = Player::O,
            Player::O => self.current_player = Player::X,
            Player::None => panic!("inconceivable!")
        }
    }

    fn print_status(&self) {
        println!("{}", self);
        println!("\n\n");
        println!("Current Turn: {:?}", self.current_player)
    }

    fn check_condition(&self, cond: [usize; 3]) -> Option<Player> {
        let pieces: Vec<Player> = cond.iter().map(|c| self.board[*c]).collect();
        let xs: usize = pieces.iter().map(|p| if *p == Player::X {1} else {0}).sum();
        let os: usize = pieces.iter().map(|p| if *p == Player::O {1} else {0}).sum();

        if xs == pieces.len() {
            return Some(Player::X)
        } else if os == pieces.len() {
            return Some(Player::O)
        }

        return None
    }

    fn check_win_condition(&self) -> Option<Player> {
        let conditions: Vec<[usize; 3]> = vec![
            [0, 1, 2], [3, 4, 5], [6, 7, 8],
            [0, 3, 6], [1, 4, 7], [2, 5, 8],
            [0, 4, 8], [2, 4, 6]];

        let mut wins = conditions
            .into_iter()
            .map(|c| self.check_condition(c))
            .filter(|p| p.is_some())
            .map(|s| s.unwrap())
            .collect::<Vec<Player>>();

        let ns: usize = self.board.iter().map(|p| if *p != Player::None {1} else {0}).sum();
        if ns == self.board.len() {
            wins.push(Player::None);
        }


        wins.reverse();
        return wins.pop();
    }


    fn process_turn(&mut self, input: String) {

        println!();
        let c = match input.as_str().chars().nth(0) {
            Some(n) => n.to_digit(10),
            None => None,
        };


        match c {
            Some(n @ 1..10)  => self.do_move((n - 1) as usize),
            _ => println!("'{}' is not a valid move!\n", input)
        }

        println!("{}\n", self);

    }

    fn run(&mut self) {
        let stdin = io::stdin();
        println!("Turn {:?}", self.current_player);
        for line in stdin.lock().lines() {
            self.process_turn(line.unwrap());

            match self.check_win_condition() {
                Some(p) => {println!("{:?} is the winner!", p); return},
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
            board: [Player::None; 9],
        };

        println!("New Game!");
        game.print_status();
        game.run();
    }
}
