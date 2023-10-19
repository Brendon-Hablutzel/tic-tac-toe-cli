use std::io::stdin;
mod game;
use game::{Board, Cell, GameState, Player};

macro_rules! do_or_continue {
    ($fallible:expr) => {
        match $fallible {
            Ok(result) => result,
            Err(err) => {
                eprintln!("Error: {err}");
                continue;
            }
        }
    };
}

fn minimax(
    board: Board,
    player_to_move: Player,
    maximizing_player: Player,
    current_depth: u8,
) -> i8 {
    match board.get_state() {
        GameState::Win(p) => {
            if p == maximizing_player {
                10 - current_depth as i8
            } else {
                -10 + current_depth as i8
            }
        }
        GameState::Draw => 0,
        GameState::Ongoing => {
            let next_scores =
                board
                    .as_iter()
                    .enumerate()
                    .filter_map(|(position, cell)| match cell {
                        Cell::Player(_) => None,
                        Cell::Empty => {
                            let mut next_board = board.clone();
                            next_board
                                .make_move(player_to_move, position)
                                .expect("Minimax moves should be valid");

                            Some(minimax(
                                next_board,
                                player_to_move.opponent(),
                                maximizing_player,
                                current_depth + 1,
                            ))
                        }
                    });

            if player_to_move == maximizing_player {
                next_scores.max()
            } else {
                next_scores.min()
            }
            .expect("Minimax should produce non-empty iter of possible next moves")
        }
    }
}

fn get_best_move(board: &Board, current_player: Player) -> usize {
    board
        .as_iter()
        .enumerate()
        .filter_map(|(position, cell)| match cell {
            Cell::Player(_) => None,
            Cell::Empty => Some(position),
        })
        .max_by_key(|position| {
            let mut next_board = board.clone();
            next_board
                .make_move(current_player, *position)
                .expect("Minimax moves should be valid");
            minimax(next_board, current_player.opponent(), current_player, 0)
        })
        .expect("Board within game loop should have empty cells")
}

fn main() {
    let mut board = Board::new();
    let mut turn_number = 1;

    let result = loop {
        let player = match turn_number % 2 {
            1 => Player::One,
            0 => Player::Two,
            _ => panic!("Integer should return 1 or 0 modulo 2"),
        };

        let best_move = get_best_move(&board, player);

        println!("Turn {turn_number}, {player} to move\n{board}\nOptimal move: {best_move}");

        let mut position_input = String::new();
        
        println!("Enter a position");
        do_or_continue!(stdin().read_line(&mut position_input));

        let position_input: usize = do_or_continue!(position_input.trim().parse());

        do_or_continue!(board.make_move(player, position_input));

        println!("------------");

        match board.get_state() {
            GameState::Win(player) => break format!("Win for Player {player}"),
            GameState::Draw => break "Draw".to_owned(),
            GameState::Ongoing => (),
        }

        turn_number += 1;
    };
    println!("{board}\n{} in {turn_number} turns", result);
}
