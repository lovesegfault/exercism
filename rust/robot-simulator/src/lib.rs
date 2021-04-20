#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn turn_right(&mut self) {
        use Direction::*;
        *self = match self {
            North => East,
            East => South,
            South => West,
            West => North,
        };
    }
    pub fn turn_left(&mut self) {
        use Direction::*;
        *self = match self {
            North => West,
            East => North,
            South => East,
            West => South,
        };
    }
}

pub struct Robot {
    position: (i32, i32),
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self {
            position: (x, y),
            direction: d,
        }
    }

    pub fn turn_right(mut self) -> Self {
        self.direction.turn_right();
        self
    }

    pub fn turn_left(mut self) -> Self {
        self.direction.turn_left();
        self
    }

    pub fn advance(mut self) -> Self {
        use Direction::*;
        let transformation = match self.direction {
            North => (0, 1),
            East => (1, 0),
            South => (0, -1),
            West => (-1, 0),
        };
        self.position = (
            self.position.0 + transformation.0,
            self.position.1 + transformation.1,
        );
        self
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions
            .chars()
            .filter(|&c| c == 'A' || c == 'L' || c == 'R')
            .fold(self, |robot, instruction| match instruction {
                'L' => robot.turn_left(),
                'R' => robot.turn_right(),
                'A' => robot.advance(),
                _ => unreachable!(),
            })
    }

    pub fn position(&self) -> (i32, i32) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
