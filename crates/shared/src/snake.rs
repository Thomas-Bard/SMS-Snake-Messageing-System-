use crate::types::{GridPos, MovementDirection};
use std::collections::VecDeque;

pub struct Snake {
    pub body: VecDeque<GridPos>,
    pub direction: MovementDirection,
}

impl Snake {
    pub fn new(length: i32) -> Snake {
        let mut temp = Snake {
            body: VecDeque::new(),
            direction: MovementDirection::Right,
        };
        temp.body.push_back(GridPos {
            y: length,
            x: length,
        });
        for _ in 0..length {
            temp.grow();
        }
        return temp;
    }
    pub fn step(&mut self) {
        let mut head_pos: GridPos = GridPos {
            x: self.body.front().unwrap_or(&GridPos { x: -1, y: -1 }).x,
            y: self.body.front().unwrap_or(&GridPos { x: -1, y: -1 }).y,
        };
        let direction_vector = match self.direction {
            MovementDirection::Up => GridPos { y: -1, x: 0 },
            MovementDirection::Down => GridPos { y: 1, x: 0 },
            MovementDirection::Left => GridPos { y: 0, x: -1 },
            MovementDirection::Right => GridPos { y: 0, x: 1 },
        };

        head_pos += direction_vector;
        self.body.push_front(head_pos);
        self.body.pop_back();
    }
    pub fn grow(&mut self) {
        let change_vector = match self.direction {
            MovementDirection::Down => GridPos { x: 0, y: -1 },
            MovementDirection::Left => GridPos { x: 1, y: 0 },
            MovementDirection::Right => GridPos { x: -1, y: 0 },
            MovementDirection::Up => GridPos { x: 0, y: 1 },
        };
        let last_pos: GridPos = *self.body.back().unwrap_or(&GridPos { y: -1, x: -1 });
        self.body.push_back(last_pos + change_vector);
    }
}
