use macroquad::prelude::{screen_height, screen_width};
use std::ops::{Add, AddAssign};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GridPos {
    pub y: i32,
    pub x: i32,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum MovementDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FoodManager {
    pub plate: Vec<GridPos>,
    grid_size: f32,
}

impl FoodManager {
    pub fn new(amount: u32, gs: f32) -> FoodManager {
        let mut temp = FoodManager {
            plate: Vec::new(),
            grid_size: gs,
        };
        let max_x = ((screen_width() / gs) - 1.0) as u32;
        let max_y = ((screen_height() / gs) - 1.0) as u32;
        for _ in 0..amount {
            temp.plate.push(GridPos {
                y: rand::random_range(0..max_y) as i32,
                x: rand::random_range(0..max_x) as i32,
            });
        }
        temp
    }
    pub fn eat(&mut self, pos: &GridPos) -> bool {
        let mut idx = 0;
        for i in &self.plate {
            if *i == *pos {
                self.plate.swap_remove(idx);
                return true;
            }
            idx += 1;
        }
        false
    }
}

impl Add for GridPos {
    type Output = GridPos;
    fn add(self, rhs: Self) -> Self::Output {
        GridPos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for GridPos {
    fn add_assign(&mut self, rhs: Self) {
        *self = GridPos {
            y: self.y + rhs.y,
            x: self.x + rhs.x,
        }
    }
}
