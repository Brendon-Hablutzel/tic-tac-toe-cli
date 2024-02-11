use anyhow;
use std::io::{stdin, stdout, Write};
mod game;
use game::{Board, GameState, Player};
mod solver;

macro_rules! do_or_continue {
    ($fallible:expr) => {
        match $fallible {
            Ok(result) => result,
            Err(err) => {
                eprintln!("Error: {err}\n");
                continue;
            }
        }
    };
}

fn play(against_computer: bool) {
    println!("Board positions:");
    println!("012\n345\n678\n");

    let mut board = Board::new();
    let mut turn_number = 1;

    let result = loop {
        let player = match turn_number % 2 {
            1 => Player::One,
            0 => Player::Two,
            _ => panic!("an integer should return 1 or 0 modulo 2"),
        };

        let position_input: usize = if against_computer && player == Player::Two {
            let best_move = solver::get_best_move(&board, player);

            println!("Turn {turn_number}, computer played position {best_move} for {player}");

            best_move
        } else {
            println!("Turn {turn_number}, {player} to move\n{board}");

            let mut position_input = String::new();

            print!("Enter a position: ");
            // necessary so that the print! call is printed immediately
            stdout().flush().expect("should be able to flush stdout");

            do_or_continue!(stdin().read_line(&mut position_input));

            do_or_continue!(position_input.trim().parse())
        };

        do_or_continue!(board.make_move(player, position_input));

        println!();

        match board.get_state() {
            GameState::Win(player) => break format!("Win for Player {player}"),
            GameState::Draw => break "Draw".to_owned(),
            GameState::Ongoing => (),
        }

        turn_number += 1;
    };
    println!("Final board:\n{board}\n{} in {turn_number} turns", result);
}

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args();

    args.next(); // skip first command line argument

    let Some(game_mode) = args.next() else {
        anyhow::bail!("an argument for game mode is required")
    };

    match game_mode.as_str() {
        "player" => play(false),
        "computer" => play(true),
        invalid => anyhow::bail!(
            "invalid game mode: expected 'player' or 'computer' got {}",
            invalid
        ),
    }

    Ok(())
}
