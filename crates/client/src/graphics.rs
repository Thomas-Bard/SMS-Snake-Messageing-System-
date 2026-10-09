use macroquad::prelude::*;
use shared::snake::Snake;
use shared::types::{FoodManager, GridPos};

fn render_square(pos: &GridPos, grid_size: f32, color: Color) {
    let absolute_x = grid_size * pos.x as f32;
    let absolute_y = grid_size * pos.y as f32;

    draw_rectangle(
        absolute_x,
        absolute_y,
        grid_size - 2.0,
        grid_size - 2.0,
        color,
    );
}

pub fn render_snake(snake: &Snake, grid_size: f32) {
    for i in &snake.body {
        render_square(i, grid_size, GREEN);
    }
}

pub fn render_food(food: &FoodManager, grid_size: f32) {
    for i in &food.plate {
        render_square(i, grid_size, RED);
    }
}
