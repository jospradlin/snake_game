
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

// Use `wee_alloc` as the global allocator.
extern crate wee_alloc;
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;


// Snake Direction
#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    RIGHT,
    LEFT,
}


// Snake
#[wasm_bindgen]
#[derive(Clone)]
struct SnakeCell(usize);

#[wasm_bindgen]
struct Snake {
    body: Vec<SnakeCell>,
    direction: Direction,
    size: usize,
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
            direction: Direction::DOWN,
            size,
        }
    }

}


// World
#[wasm_bindgen]
struct World {
    width: usize,
    size: usize,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize, snake_index: usize, starting_size: usize) -> World {
        World {
            width,
            size: width * width,
            snake: Snake::new(snake_index, starting_size),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn size(&self) -> usize {
        self.size
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

    pub fn step(&mut self) {
        let temp = self.snake.body.clone();
        let update_cell = self.gen_next_snake_cell(0);
        self.snake.body[0] = update_cell;

        let snake_length = self.snake.body.len();
        for i in 1..snake_length {
            self.snake.body[i] = SnakeCell(temp[i-1].0);
        }
    }

    fn gen_next_snake_cell(&self, snake_index: usize) -> SnakeCell {
        //let snake_cells = self.snake_cells();
        //let snake_index = self.snake_head_index();

        let snake_index = self.snake.body[snake_index].0;
        let row = snake_index / self.width;
        let col = snake_index % self.width;

        // If needed - review 77 and 78 to align to Training code
        return match self.snake.direction {
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
        if self.snake.direction == Direction::UP && new_direction == Direction::DOWN {
            return;
        } else if self.snake.direction == Direction::DOWN && new_direction == Direction::UP {
            return;
        } else if self.snake.direction == Direction::LEFT && new_direction == Direction::RIGHT {
            return;
        } else if self.snake.direction == Direction::RIGHT && new_direction == Direction::LEFT {
            return;
        } else {
            self.snake.direction = new_direction;
        }
    }
}






// wasm-pack build --target web