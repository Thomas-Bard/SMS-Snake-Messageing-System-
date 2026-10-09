use macroquad::prelude::*;
use shared::{snake, types::MovementDirection};

pub fn process_inputs(snake: &mut snake::Snake) {
    if is_key_down(KeyCode::Up) && snake.direction != MovementDirection::Down {
        snake.direction = MovementDirection::Up;
        return;
    }
    if is_key_down(KeyCode::Down) && snake.direction != MovementDirection::Up {
        snake.direction = MovementDirection::Down;
        return;
    }

    if is_key_down(KeyCode::Left) && snake.direction != MovementDirection::Right {
        snake.direction = MovementDirection::Left;
        return;
    }
    if is_key_down(KeyCode::Right) && snake.direction != MovementDirection::Left {
        snake.direction = MovementDirection::Right;
        return;
    }
}
