import init, { World, Direction } from "snake_game";
import { rnd } from "./utils/rnd";
import { wasm } from "webpack";

init().then(wasm => {
  const CELL_SIZE = 20;
  const WORLD_WIDTH = 16;
  const snakeSpawnIndex = rnd(WORLD_WIDTH * WORLD_WIDTH);
  const STARTING_SNAKE_SIZE = 3;

  const world = World.new(WORLD_WIDTH, snakeSpawnIndex, STARTING_SNAKE_SIZE);
  const worldWidth = world.width();

  const canvas = <HTMLCanvasElement> document.getElementById("snake-canvas");
  const ctx = canvas.getContext("2d")
  
  canvas.height = worldWidth * CELL_SIZE;
  canvas.width =  worldWidth * CELL_SIZE;

  document.addEventListener("keydown", e => {
    switch(e.code) {
      case "ArrowUp":
      case "KeyW":  
      case "Numpad8": 
        world.handle_input(Direction.UP);
        break; 
      case "ArrowDown":
      case "KeyS":  
      case "Numpad2": 
        world.handle_input(Direction.DOWN);
        break; 
      case "ArrowLeft":
      case "KeyA":  
      case "Numpad4": 
        world.handle_input(Direction.LEFT);
        break;     
      case "ArrowRight":
      case "KeyD":  
      case "Numpad6": 
        world.handle_input(Direction.RIGHT);
        break;      
    }
  });


  function drawWorld() {
    ctx.beginPath();

    for (let x = 0; x < worldWidth + 1; x++) {
      ctx.moveTo(CELL_SIZE * x, 0);
      ctx.lineTo(CELL_SIZE * x, worldWidth * CELL_SIZE);
    }

    for (let y = 0; y < worldWidth + 1; y++) {
      ctx.moveTo(0, CELL_SIZE * y );
      ctx.lineTo(worldWidth * CELL_SIZE, CELL_SIZE * y);
    }

    ctx.stroke();
  }

  function drawSnake() {
    const snakeCellPtr = world.snake_cells();
    const snakeLength = world.snake_length();
    const snakeCells = new Uint32Array(
      wasm.memory.buffer,
      snakeCellPtr,
      snakeLength,
    )

    snakeCells.forEach((cellIndex, i) => {
      const col = cellIndex % worldWidth;
      const row = Math.floor(cellIndex / worldWidth);
      
      ctx.fillStyle = i === 0? "#121276" : "#5555ff";

      ctx.beginPath();
      ctx.fillRect(
        col * CELL_SIZE,
        row * CELL_SIZE,
        CELL_SIZE,
        CELL_SIZE
      );
    });

    //OLD -const snakeIndex = world.snake_head_index();

    ctx.stroke();
  }

  function draw_reward_cell() {
    const rewardCellLocation = world.reward_cell();
    const reward_col = rewardCellLocation % worldWidth;
    const reward_row = Math.floor(rewardCellLocation / worldWidth);
      
      ctx.fillStyle = "#ff0000"; 

      ctx.beginPath();
      ctx.fillRect(
        reward_col * CELL_SIZE,
        reward_row * CELL_SIZE,
        CELL_SIZE,
        CELL_SIZE
      );

    //OLD -const snakeIndex = world.snake_head_index();

    ctx.stroke();
  }
  

  function render() {
    drawWorld();
    drawSnake();
    draw_reward_cell();
  }

  function update() {
    const fps = 5;

    //setInterval (many times) vs setTimeout (once)
    setTimeout( () => {
      ctx.clearRect(0,0,canvas.width, canvas.height);
      world.step();
      render();

      // the method takes a callback to be invoked before next re-render
      requestAnimationFrame(update);
    }, 1000 / fps);
  }

  render();
  update();
});