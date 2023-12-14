
use wasm_bindgen::prelude::*;
use wee_alloc::WeeAlloc;

// Use `wee_alloc` as the global allocator.
extern crate wee_alloc;
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

// Snake
#[wasm_bindgen]
struct SnakeCell(usize);

#[wasm_bindgen]
struct Snake {
    body: Vec<SnakeCell>,
}

#[wasm_bindgen]
impl Snake {
    pub fn new(spawn_index: usize) -> Snake {
        Snake {
            body: vec!(SnakeCell(spawn_index)),
        }
    }

}


// World
#[wasm_bindgen]
struct World {
    width: usize,
    snake: Snake,
}

#[wasm_bindgen]
impl World {
    pub fn new(width: usize) -> World {
        World {
            width,
            snake: Snake::new(10),
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn snake_head_index(&self) -> usize {
        self.snake.body[0].0
    }
}




// wasm-pack build --target web