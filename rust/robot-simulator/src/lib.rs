// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    direction: Direction,
    x: i32,
    y: i32
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        return Robot {
            direction: d,
            x,
            y
        }
    }

    #[must_use]
    pub fn turn_right(mut self) -> Self {
        use Direction::*;
        match self.direction {
            North => self.direction = East,
            South => self.direction = West,
            East => self.direction = South,
            West => self.direction = North
        }
        return self;
    }

    #[must_use]
    pub fn turn_left(mut self) -> Self {
        use Direction::*;
        match self.direction {
            North => self.direction = West,
            South => self.direction = East,
            East => self.direction = North,
            West => self.direction = South
        }
        return self;
    }

    #[must_use]
    pub fn advance(mut self) -> Self {
        use Direction::*;
        match self.direction {
            North => self.y += 1,
            South => self.y -= 1,
            East => self.x += 1,
            West => self.x -= 1
        }
        return self;
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;
        for c in instructions.chars() {
            match c {
                'L' => { robot = robot.turn_left(); },
                'R' => { robot = robot.turn_right(); },
                _ => { robot = robot.advance(); }
            }
        }
        return robot;
    }

    pub fn position(&self) -> (i32, i32) {
        return (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        return &self.direction;
    }
}
