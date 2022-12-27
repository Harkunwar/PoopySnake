use lol_alloc::{FreeListAllocator, LockedAllocator};
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOCATOR: LockedAllocator<FreeListAllocator> =
    LockedAllocator::new(FreeListAllocator::new());

#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
pub struct SnakeCell(usize);
struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

impl Snake {
    fn new(spawn_index: usize, size: usize) -> Snake {
        let mut body: Vec<SnakeCell> = vec![];
        for i in 0..size {
            body.push(SnakeCell(spawn_index - i));
        }
        Snake {
            body,
            direction: Direction::Right,
        }
    }
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    size: usize,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_spawn_index: usize) -> World {
        World {
            width,
            size: width * width,
            snake: Snake::new(snake_spawn_index, 3),
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn snake_head_index(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn set_snake_direction(&mut self, direction: Direction) {
        self.snake.direction = direction;
    }

    pub fn get_snake_length(&self) -> usize {
        self.snake.body.len()
    }

    pub fn get_snake_cell_pointer(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    pub fn step(&mut self) {
        let next_cell = self.generate_next_snake_cell();
        self.snake.body[0] = next_cell;
    }

    fn generate_next_snake_cell(&self) -> SnakeCell {
        let snake_index = self.snake_head_index();
        let row = snake_index / self.width;

        match self.snake.direction {
            Direction::Right => SnakeCell((row * self.width) + (snake_index + 1) % self.width),
            Direction::Left => SnakeCell((row * self.width) + (snake_index - 1) % self.width),
            Direction::Up => SnakeCell((snake_index - self.width) % self.size),
            Direction::Down => SnakeCell((snake_index + self.width) % self.size),
        }
    }
}
