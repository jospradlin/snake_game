
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

// Use `wee_alloc` as the global allocator.
extern crate wee_alloc;
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

#[wasm_bindgen(module = "/www/utils/rnd.js")]
extern {
    fn rnd(max: usize) -> usize;
}


// Snake Direction
#[wasm_bindgen]
#[derive(PartialEq, Debug)]
pub enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}


#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum GameStatus {
    WON,
    LOST,
    PLAYED,
}


// Snake
#[wasm_bindgen]
#[derive(Clone, Copy, PartialEq)]
struct SnakeCell(usize);

#[wasm_bindgen]
struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
}

#[wasm_bindgen]
impl Snake {
    pub fn new(spawn_index: usize, size: usize) -> Snake {

        let mut body = vec!();
        for i in 0..size {
            body.push(SnakeCell(spawn_index - i));
        }

//: vec!(SnakeCell(spawn_index))
        Snake {
            body,
            direction: Direction::RIGHT,
        }
    }
}


// World
#[wasm_bindgen]
struct World {
    width: usize,
    size: usize,
    snake: Snake,
    next_cell: Option<SnakeCell>,
    reward_cell: usize,
    status: Option<GameStatus>,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_index: usize, starting_size: usize) -> World {

        let size = width * width;
        let snake = Snake::new(snake_index, starting_size);

        World {
            width,
            size,
            reward_cell: World::set_random_reward_cell(size, &snake.body),
            snake,
            next_cell: None,
            status: None,
        }
    }

    pub fn start_game(&mut self) {
        self.status = Some(GameStatus::PLAYED);
    }

    pub fn status(&self) -> Option<GameStatus> {
        self.status
    }

    pub fn game_status_text(&self) -> String {
        match self.status {
            Some(GameStatus::WON) => String::from("You have won!"),
            Some(GameStatus::LOST) => String::from("You have lost!"),
            Some(GameStatus::PLAYED) => String::from("Playing"),
            None => String::from("No Status"),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn reward_cell(&self) -> usize {
        self.reward_cell
    }

    pub fn snake_head_index(&self) -> usize {
        self.snake.body[0].0
    }

    // cannot return a reference to JS because of borrowing rules
    // pub fn snake_cells(&self) -> &Vec<SnakeCell> {
    //     &self.snake.body
    // }

    // *const is a raw pointer
    //    borrowing rules doesn't apply to it
    pub fn snake_cells(&self) -> *const SnakeCell {
        self.snake.body.as_ptr()
    }

    pub fn snake_length(&self) -> usize {
        self.snake.body.len()
    }

    fn set_random_reward_cell(max_size: usize, snake_body: &Vec<SnakeCell>) -> usize {

        let mut reward_cell;
        loop {
            reward_cell = rnd(max_size);
            if !snake_body.contains(&SnakeCell(reward_cell)) { break;}
        }
        
        reward_cell
    }

    pub fn step(&mut self) {

        match self.status {
            Some(GameStatus::PLAYED) => {
                let temp = self.snake.body.clone();

                match self.next_cell {
                    Some(cell) => {
                        self.snake.body[0] = cell;
                        self.next_cell = None;
                    },
                    None => {
                        self.snake.body[0] = self.gen_next_snake_cell(&self.snake.direction);
                    }
                }
        
                let snake_length = self.snake.body.len();
                for i in 1..snake_length {
                    self.snake.body[i] = SnakeCell(temp[i-1].0);
                }
        
                if self.reward_cell == self.snake_head_index() {
                    if self.snake_length() < self.size {
                        self.reward_cell = World::set_random_reward_cell(self.size, &self.snake.body)
                    } else {
                        self.reward_cell = 1000;
                    }
                    self.snake.body.push(SnakeCell(self.snake.body[1].0));
                    
                    //self.set_random_reward_cell();
                }
            },
            _ => {},
        }

    }

    fn gen_next_snake_cell(&self, direction: &Direction) -> SnakeCell {
        //let snake_cells = self.snake_cells();
        //let snake_index = self.snake_head_index();

        let snake_index = self.snake.body[0].0;
        let row = snake_index / self.width;

        // If needed - review 77 and 78 to align to Training code
        return match direction {
            Direction::UP => {
                let treshold = snake_index - (row * self.width);
                if snake_index == treshold {
                    SnakeCell((self.size - self.width) + treshold)
                } else {
                    SnakeCell(snake_index - self.width)
                }
            },
            Direction::DOWN => {
                let treshold = snake_index + ((self.width - row) * self.width);
                if snake_index + self.width == treshold {
                    SnakeCell(treshold - ((row + 1) * self.width))
                } else {
                    SnakeCell(snake_index + self.width)
                }
            },
            Direction::LEFT => { 
                let threshold = row * self.width;
                if snake_index == threshold {
                    SnakeCell(threshold + (self.width - 1))
                } else {
                    SnakeCell(snake_index - 1)
                }    
            },
            
            Direction::RIGHT => { 
                let threshold = (row + 1) * self.width;
                if snake_index + 1 == threshold {
                    SnakeCell(threshold - self.width)
                } else {
                    SnakeCell(snake_index + 1)
                }    
            },
        }
    }

    pub fn handle_input(&mut self, new_direction: Direction) {
        let next_cell = self.gen_next_snake_cell(&new_direction);

        if self.snake.body[1].0 == next_cell.0 { return; } // Ensure Snake is never 1 node only

        self.next_cell = Some(next_cell);
        self.snake.direction = new_direction;
    }
}






// wasm-pack build --target web