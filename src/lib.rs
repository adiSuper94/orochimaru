mod utils;

use std::collections::HashSet;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct CellPosition {
    x: usize,
    y: usize,
}

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Universe {
    width: usize,
    height: usize,
    food: HashSet<CellPosition>,
    snake: Vec<CellPosition>,
    snake_direction: Direction,
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Universe {
    pub fn new(width: usize, height: usize) -> Self {
        let snake: Vec<CellPosition> = vec![CellPosition { x: 0, y: 0 }];
        Self {
            width,
            height,
            snake,
            food: HashSet::new(),
            snake_direction: Direction::Right,
        }
    }
    pub fn tick(&mut self) {
        let mut next_head_position: CellPosition = self.snake[0];
        match &self.snake_direction {
            Direction::Up => next_head_position.y += 1,
            Direction::Down => next_head_position.y -= 1,
            Direction::Left => next_head_position.x -= 1,
            Direction::Right => next_head_position.x += 1,
        }
        next_head_position.x %= self.width;
        next_head_position.y %= self.height;

        self.snake.remove(self.snake.len() - 1);
        self.snake.insert(0, next_head_position);
    }

    pub fn set_snake_direction(&mut self, dir: Direction) {
        match dir {
            Direction::Up => {
                if self.snake_direction == Direction::Down {
                    return;
                }
            }
            Direction::Down => {
                if self.snake_direction == Direction::Up {
                    return;
                }
            }
            Direction::Left => {
                if self.snake_direction == Direction::Right {
                    return;
                }
            }
            Direction::Right => {
                if self.snake_direction == Direction::Left {
                    return;
                }
            }
        }
        self.snake_direction = dir;
    }
}
