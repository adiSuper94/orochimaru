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
    food_pos: Vec<u32>,
    snake: Vec<CellPosition>,
    snake_pos: Vec<u32>,
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

#[wasm_bindgen]
impl Universe {
    pub fn new(width: usize, height: usize) -> Self {
        let snake: Vec<CellPosition> = vec![CellPosition { x: 0, y: 0 }];
        Self {
            width,
            height,
            snake,
            snake_pos: vec![0, 0],
            food: HashSet::new(),
            food_pos: Vec::new(),
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

        if !self.food.contains(&next_head_position) {
            self.snake.remove(self.snake.len() - 1);
        } else {
            self.food.remove(&next_head_position);
        }
        self.snake.insert(0, next_head_position);
        self.gen_food();
        self.aos2soa();
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

    pub fn get_snake_len(&self) -> usize {
        self.snake.len()
    }

    pub fn get_snake_position(&self) -> *const u32 {
        self.snake_pos.as_ptr()
    }

    pub fn get_food_count(&self) -> usize {
        self.food.len()
    }

    pub fn get_food_position(&self) -> *const u32 {
        self.food_pos.as_ptr()
    }

    fn gen_food(&mut self) {
        if self.food.len() > 5 {
            return;
        }
        if rand::random::<f32>() < 0.1 || self.food.is_empty() {
            let mut x: usize = rand::random();
            let mut y: usize = rand::random();
            x %= self.width;
            y %= self.height;
            self.food.insert(CellPosition { x, y });
        }
    }

    fn aos2soa(&mut self) {
        let mut soc_snake: Vec<u32> = Vec::new();
        for position in self.snake.iter() {
            soc_snake.push(position.x as u32);
            soc_snake.push(position.y as u32);
        }
        self.snake_pos = soc_snake;
        let mut soc_food: Vec<u32> = Vec::new();
        for position in self.food.iter() {
            soc_food.push(position.x as u32);
            soc_food.push(position.y as u32);
        }
        self.food_pos = soc_food;
    }
}
