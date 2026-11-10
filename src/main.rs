use macroquad::prelude::*;
use std::collections::VecDeque;

const GRID_SIZE: f32 = 20.0;
const GRID_WIDTH: i32 = 40;
const GRID_HEIGHT: i32 = 30;
const MOVE_INTERVAL: f64 = 0.15;

#[derive(PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    body: VecDeque<(i32, i32)>,
    direction: Direction,
    next_direction: Direction,
}

impl Snake {
    fn new() -> Self {
        let mut body = VecDeque::new();
        body.push_front((10, 10));
        body.push_back((9, 10));
        body.push_back((8, 10));

        Self {
            body,
            direction: Direction::Right,
            next_direction: Direction::Right,
        }
    }
}

#[macroquad::main("Rust Snake")]
async fn main() {
    let mut snake = Snake::new();
    let mut food = (15, 15);
    let mut last_move = 0.0;
    let mut score = 0;
    let mut game_over = false;

    loop {
        let now = get_time();

        if !game_over {
            if is_key_pressed(KeyCode::Up) && snake.direction != Direction::Down {
                snake.next_direction = Direction::Up;
            }
            if is_key_pressed(KeyCode::Down) && snake.direction != Direction::Up {
                snake.next_direction = Direction::Down;
            }
            if is_key_pressed(KeyCode::Left) && snake.direction != Direction::Right {
                snake.next_direction = Direction::Left;
            }
            if is_key_pressed(KeyCode::Right) && snake.direction != Direction::Left {
                snake.next_direction = Direction::Right;
            }
        } else {
            if is_key_pressed(KeyCode::Space) {
                snake = Snake::new();
                food = (15, 15);
                score = 0;
                game_over = false;
            }
        }

        if !game_over && now - last_move > MOVE_INTERVAL {
            last_move = now;
            snake.direction = snake.next_direction;

            let mut new_head = *snake.body.front().unwrap();
            match snake.direction {
                Direction::Up => new_head.1 -= 1,
                Direction::Down => new_head.1 += 1,
                Direction::Left => new_head.0 -= 1,
                Direction::Right => new_head.0 += 1,
            }

            if new_head.0 < 0 || new_head.0 >= GRID_WIDTH || new_head.1 < 0 || new_head.1 >= GRID_HEIGHT {
                game_over = true;
            }

            if snake.body.contains(&new_head) {
                game_over = true;
            }

            if !game_over {
                snake.body.push_front(new_head);

                if new_head == food {
                    score += 1;
                    loop {
                        let new_food = (
                            macroquad::rand::gen_range(0, GRID_WIDTH),
                            macroquad::rand::gen_range(0, GRID_HEIGHT),
                        );
                        if !snake.body.contains(&new_food) {
                            food = new_food;
                            break;
                        }
                    }
                } else {
                    snake.body.pop_back();
                }
            }
        }

        clear_background(BLACK);

        draw_rectangle(
            food.0 as f32 * GRID_SIZE,
            food.1 as f32 * GRID_SIZE,
            GRID_SIZE - 2.0,
            GRID_SIZE - 2.0,
            RED,
        );

        for (i, &(x, y)) in snake.body.iter().enumerate() {
            let color = if i == 0 { LIME } else { GREEN };
            draw_rectangle(
                x as f32 * GRID_SIZE,
                y as f32 * GRID_SIZE,
                GRID_SIZE - 2.0,
                GRID_SIZE - 2.0,
                color,
            );
        }

        draw_text(&format!("Score: {}", score), 10.0, 30.0, 30.0, WHITE);

        if game_over {
            draw_text("GAME OVER", screen_width() / 2.0 - 100.0, screen_height() / 2.0, 40.0, RED);
            draw_text("Press SPACE to restart", screen_width() / 2.0 - 120.0, screen_height() / 2.0 + 40.0, 20.0, WHITE);
        }

        next_frame().await;
    }
}
