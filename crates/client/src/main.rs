mod graphics;
mod inputs;
mod simulation_time;

use crate::graphics::{render_food, render_snake};
use crate::inputs::process_inputs;
use crate::simulation_time::Simulation;
use macroquad::prelude::*;
use shared::snake::Snake;
use shared::types::FoodManager;

#[macroquad::main("SMS SnakeMessagingSystem")]
async fn main() {
    let mut snake = Snake::new(5);
    let mut sim = Simulation::new(10.0);
    let mut food = FoodManager::new(20, 20.0);
    loop {
        clear_background(BLACK);
        process_inputs(&mut snake);
        if sim.tick(get_frame_time()) {
            snake.step();
            if food.eat(snake.body.front().unwrap()) {
                snake.grow();
            }
        }
        render_snake(&snake, 20.0);
        render_food(&food, 20.0);
        next_frame().await;
    }
}
