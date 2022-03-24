use rand::thread_rng;
use rand::prelude::SliceRandom;
use crate::snake::game::Axis::{Horizontal, Vertical};
use crate::snake::game::Direction::{Down, Up, Left, Right};

#[derive(Debug)]
pub struct GameState {
    direction: Option<Direction>,
    head_position: Position,
    tail_positions: Vec<Position>,
    apple_position: Position,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            direction: None,
            head_position: Position { row: 0, col: 0 },
            tail_positions: vec![],
            apple_position: Position { row: -1, col: -1 },
        }
    }

    pub fn is_end(&self) -> bool {
        self.get_score() >= 100 || self.is_out_of_bound() || self.is_suicide()
    }

    pub fn get_score(&self) -> i32 {
        self.tail_positions.len() as i32
    }

    pub fn is_out_of_bound(&self) -> bool {
        let pos = self.head_position;
        pos.row < -5 || pos.row > 4 || pos.col < -5 || pos.col > 4
    }

    pub fn is_suicide(&self) -> bool {
        self.tail_positions.contains(&self.head_position)
    }

    pub fn get_apple_position(&self) -> Position {
        self.apple_position
    }

    pub fn get_snake_positions(&self) -> Vec<Position> {
        let mut vec = self.tail_positions.clone();
        vec.push(self.head_position);
        vec
    }

    pub fn set_direction(&mut self, direction: Direction) {
        if self.can_turn_to(direction) {
            self.direction = Some(direction);
        }
    }

    fn can_turn_to(&self, d: Direction) -> bool {
        if let Some(direction) = self.direction {
            direction.to_axis() != d.to_axis()
        } else {
            true
        }
    }

    pub fn regenerate_apple(&mut self) {
        let snake_positions = self.get_snake_positions();
        let mut rng = thread_rng();

        let remaining_space = (-5..=4).collect::<Vec<i32>>()
            .into_iter()
            .map(move |row|
                (-5..=4).collect::<Vec<i32>>()
                    .into_iter()
                    .map(move |col|
                        Position::new(row, col)
                    )
            )
            .flatten()
            .filter(move |pos| !snake_positions.contains(pos))
            .collect::<Vec<_>>();

        let new_position = remaining_space.choose(&mut rng);

        if let Some(new_position) = new_position {
            self.apple_position = *new_position;
        }
    }

    pub fn tick(&mut self) {
        let apple_eaten = self.head_position == self.apple_position;
        self.tail_positions.push(self.head_position);
        if apple_eaten {
            self.regenerate_apple();
        } else {
            self.tail_positions.remove(0);
        }

        let p = self.head_position;
        self.head_position = match self.direction {
            Some(Direction::Up) => Position::new(p.row - 1, p.col),
            Some(Direction::Down) => Position::new(p.row + 1, p.col),
            Some(Direction::Left) => Position::new(p.row, p.col - 1),
            Some(Direction::Right) => Position::new(p.row, p.col + 1),
            None => self.head_position
        };

    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Axis {
    Horizontal, Vertical
}

#[derive(Copy, Clone, Debug)]
pub enum Direction {
    Up, Down, Left, Right
}

impl Direction {
    pub fn to_axis(&self) -> Axis {
        match self {
            self::Up | self::Down => Horizontal,
            self::Left | self::Right => Vertical
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Position {
    pub row: i32,
    pub col: i32,
}

impl Position {
    fn new(row: i32, col: i32) -> Position {
        Position { row, col }
    }
}
