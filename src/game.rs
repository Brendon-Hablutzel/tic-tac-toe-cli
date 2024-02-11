use anyhow;
use std::fmt::Display;

#[derive(Clone, Copy, PartialEq)]
pub enum Player {
    One,
    Two,
}

impl Player {
    pub fn opponent(&self) -> Player {
        match self {
            Player::One => Player::Two,
            Player::Two => Player::One,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Player::One => "X",
                Player::Two => "O",
            }
        )
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Player(Player),
    Empty,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Cell::Player(p) => write!(f, "{p}"),
            Cell::Empty => write!(f, "#"),
        }
    }
}

pub enum GameState {
    Ongoing,
    Draw,
    Win(Player),
}

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GameState::Ongoing => "Ongoing",
                GameState::Draw => "Draw",
                GameState::Win(Player::One) => "Win for Player One (X)",
                GameState::Win(Player::Two) => "Win for Player Two (O)",
            }
        )
    }
}

#[derive(Clone)]
pub struct Board([Cell; 9]);

impl Board {
    pub fn new() -> Self {
        Board([Cell::Empty; 9])
    }

    pub fn make_move(&mut self, player: Player, position: usize) -> anyhow::Result<()> {
        if position > 8 {
            anyhow::bail!("invalid position: too large");
        }

        if self.0[position] != Cell::Empty {
            anyhow::bail!("invalid position: already taken");
        }

        self.0[position] = Cell::Player(player);

        Ok(())
    }

    pub fn get_state(&self) -> GameState {
        let winning_triples: [(usize, usize, usize); 8] = [
            (0, 1, 2), // top row
            (3, 4, 5), // middle row
            (6, 7, 8), // bottom row
            (0, 3, 6), // left column
            (1, 4, 7), // middle column
            (2, 5, 8), // right column
            (0, 4, 8), // left top to right bottom
            (2, 4, 6), // right top to left bottom
        ];

        for (first, second, third) in winning_triples {
            if self.0[first] == Cell::Player(Player::One)
                && self.0[second] == Cell::Player(Player::One)
                && self.0[third] == Cell::Player(Player::One)
            {
                return GameState::Win(Player::One);
            }

            if self.0[first] == Cell::Player(Player::Two)
                && self.0[second] == Cell::Player(Player::Two)
                && self.0[third] == Cell::Player(Player::Two)
            {
                return GameState::Win(Player::Two);
            }
        }

        if self.0.contains(&Cell::Empty) {
            return GameState::Ongoing;
        }

        GameState::Draw
    }

    pub fn as_iter<'a>(&'a self) -> impl Iterator<Item = &'a Cell> {
        self.0.iter()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let board = self.0;
        write!(
            f,
            "{}{}{}\n{}{}{}\n{}{}{}",
            board[0],
            board[1],
            board[2],
            board[3],
            board[4],
            board[5],
            board[6],
            board[7],
            board[8]
        )
    }
}
