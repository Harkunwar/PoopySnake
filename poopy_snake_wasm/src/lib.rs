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

#[wasm_bindgen(module = "/js/random.js")]
extern "C" {
    fn random(max: usize) -> usize;
}
#[derive(Clone, PartialEq, Copy)]
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
    next_cell: Option<SnakeCell>,
    reward_cell: usize,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_spawn_index: usize) -> World {
        let snake = Snake::new(snake_spawn_index, 3);
        let size = width * width;

        World {
            width,
            size,
            reward_cell: World::generate_reward_cell(size, &snake.body),
            snake,
            next_cell: None,
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_snake_head_index(&self) -> usize {
        self.snake.body[0].0
    }

    pub fn set_snake_direction(&mut self, direction: Direction) {
        let next_cell = self.generate_next_snake_cell(&direction);
        if self.snake.body[1] == next_cell {
            return;
        }
        self.next_cell = Some(next_cell);
        self.snake.direction = direction;
    }

    pub fn get_snake_length(&self) -> usize {
        self.snake.body.len()
    }

    pub fn get_snake_cell_pointer(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    pub fn get_reward_cell(&self) -> usize {
        self.reward_cell
    }

    pub fn step(&mut self) {
        let snake_length = self.snake.body.len();
        for i in (1..snake_length).rev() {
            self.snake.body[i] = self.snake.body[i - 1]
        }

        match self.next_cell {
            Some(cell) => {
                self.snake.body[0] = cell;
                self.next_cell = None;
            }
            None => {
                self.snake.body[0] = self.generate_next_snake_cell(&self.snake.direction);
            }
        }

        if self.reward_cell == self.get_snake_head_index() {
            self.snake.body.push(self.snake.body[1]);
            self.reward_cell = World::generate_reward_cell(self.size, &self.snake.body);
        }
    }

    fn generate_reward_cell(max: usize, snake_body: &Vec<SnakeCell>) -> usize {
        let mut reward_cell;
        loop {
            reward_cell = random(max);
            if !snake_body.contains(&SnakeCell(reward_cell)) {
                break;
            }
        }
        reward_cell
    }

    fn generate_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        let snake_index = self.get_snake_head_index();
        let row = snake_index / self.width;

        match direction {
            Direction::Right => {
                let threshold = (row + 1) * self.width;
                if snake_index + 1 == threshold {
                    SnakeCell(threshold - self.width)
                } else {
                    SnakeCell(snake_index + 1)
                }
            }
            Direction::Left => {
                let threshold = row * self.width;
                if snake_index == threshold {
                    SnakeCell(threshold + self.width - 1)
                } else {
                    SnakeCell(snake_index - 1)
                }
            }
            Direction::Up => {
                let threshold = snake_index - (row * self.width);
                if snake_index == threshold {
                    SnakeCell((self.size - self.width) + threshold)
                } else {
                    SnakeCell(snake_index - self.width)
                }
            }
            Direction::Down => {
                let threshold = snake_index + ((self.width - row) * self.width);
                if snake_index + self.width == threshold {
                    SnakeCell(threshold - ((row + 1) * self.width))
                } else {
                    SnakeCell(snake_index + self.width)
                }
            }
        }
    }
}
