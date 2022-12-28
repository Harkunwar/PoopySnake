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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

fn get_random_in_range(max: usize) -> usize {
    (random() * (max as f64)) as usize
}

#[derive(Clone, PartialEq, Copy)]
pub struct SnakeCell(usize);
struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
pub enum GameStatus {
    Won,
    Lost,
    Played,
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
    reward_cell: Option<usize>,
    poop_iterations: usize,
    iterations: usize,
    poop_cell: Option<usize>,
    status: Option<GameStatus>,
    points: isize,
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
            poop_cell: None,
            poop_iterations: 50,
            snake,
            iterations: 0,
            next_cell: None,
            status: None,
            points: 0,
        }
    }

    pub fn get_points(&self) -> isize {
        self.points
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_snake_head_index(&self) -> Option<usize> {
        if self.get_snake_length() > 0 {
            Some(self.snake.body[0].0)
        } else {
            None
        }
    }

    pub fn set_snake_direction(&mut self, direction: Direction) {
        if self.get_snake_length() == 0 {
            return;
        }
        let next_cell = self.generate_next_snake_cell(&direction);
        if self.get_snake_length() > 1 && Some(self.snake.body[1]) == next_cell {
            return;
        }
        self.next_cell = next_cell;
        self.snake.direction = direction;
    }

    pub fn get_snake_length(&self) -> usize {
        self.snake.body.len()
    }

    pub fn get_snake_cell_pointer(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    pub fn get_reward_cell(&self) -> Option<usize> {
        self.reward_cell
    }

    pub fn get_poop_cell(&self) -> Option<usize> {
        self.poop_cell
    }

    pub fn get_game_status(&self) -> Option<GameStatus> {
        self.status
    }

    pub fn get_game_status_text(&self) -> String {
        match self.status {
            Some(GameStatus::Won) => String::from("You have won!"),
            Some(GameStatus::Lost) => String::from("You have lost!"),
            Some(GameStatus::Played) => String::from("Playing"),
            None => String::from("No status"),
        }
    }

    pub fn start_game(&mut self) {
        self.status = Some(GameStatus::Played);
    }

    pub fn step(&mut self) {
        match self.status {
            Some(GameStatus::Played) => {
                let snake_length = self.get_snake_length();
                for i in (1..snake_length).rev() {
                    self.snake.body[i] = self.snake.body[i - 1]
                }

                if snake_length > 0 {
                    match self.next_cell {
                        Some(cell) => {
                            self.snake.body[0] = cell;
                            self.next_cell = None;
                        }
                        None => {
                            let next_cell = self.generate_next_snake_cell(&self.snake.direction);
                            if next_cell != None {
                                self.snake.body[0] = next_cell.unwrap();
                            }
                        }
                    }
                }

                if self.get_snake_length() == 0
                    || self.snake.body[1..snake_length].contains(&self.snake.body[0])
                {
                    self.status = Some(GameStatus::Lost);
                }

                if self.reward_cell != None && self.reward_cell == self.get_snake_head_index() {
                    if self.get_snake_length() < self.size {
                        self.points += 1;
                        self.reward_cell = World::generate_reward_cell(self.size, &self.snake.body);
                    } else {
                        self.reward_cell = None;
                        self.status = Some(GameStatus::Won);
                    }

                    self.snake.body.push(self.snake.body[0]);
                }

                self.iterations += 1;

                if self.poop_cell != None && self.poop_cell == self.get_snake_head_index() {
                    self.points -= 2;
                    self.iterations = self.poop_iterations;
                }

                if self.iterations == self.poop_iterations {
                    self.poop_cell = self.generate_poop();
                    if self.poop_cell == None {
                        self.status = Some(GameStatus::Lost);
                    }
                    self.iterations = 0;
                }
            }
            _ => {}
        }
    }

    fn generate_poop(&mut self) -> Option<usize> {
        self.snake.body.pop();
        let poop_cell_option = self.snake.body.pop();
        match poop_cell_option {
            Some(cell) => Some(cell.0),
            None => None,
        }
    }

    fn generate_reward_cell(max: usize, snake_body: &Vec<SnakeCell>) -> Option<usize> {
        let mut reward_cell;
        loop {
            reward_cell = get_random_in_range(max);
            if !snake_body.contains(&SnakeCell(reward_cell)) {
                break;
            }
        }
        Some(reward_cell)
    }

    fn generate_next_snake_cell(&self, direction: &Direction) -> Option<SnakeCell> {
        if self.get_snake_length() == 0 {
            return None;
        }
        let snake_index = self.get_snake_head_index().unwrap();

        let row = snake_index / self.width;

        let next_cell = match direction {
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
        };
        Some(next_cell)
    }
}
