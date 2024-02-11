use crate::game::{Board, Cell, GameState, Player};

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

pub fn get_best_move(board: &Board, current_player: Player) -> usize {
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
