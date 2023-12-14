import init, { World } from "snake_game";

init().then(_ => {
  const CELL_SIZE = 20;
  const WORLD_WIDTH = 8;
  const snakeSpawnIndex = Math.floor(Math.random() * (WORLD_WIDTH * WORLD_WIDTH));

  const world = World.new(WORLD_WIDTH, snakeSpawnIndex);
  const worldWidth = world.width();

  const canvas = <HTMLCanvasElement> document.getElementById("snake-canvas");
  const ctx = canvas.getContext("2d")
  
  canvas.height = worldWidth * CELL_SIZE;
  canvas.width =  worldWidth * CELL_SIZE;

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
    const snakeIndex = world.snake_head_index();
    const col = snakeIndex % worldWidth;
    const row = Math.floor(snakeIndex / worldWidth);
    
    ctx.beginPath();
    ctx.fillRect(
      col * CELL_SIZE,
      row * CELL_SIZE,
      CELL_SIZE,
      CELL_SIZE
    );
    ctx.stroke();
  }

  function render() {
    drawWorld();
    drawSnake();
  }

  function update() {
    const fps = 5;

    //setInterval (many times) vs setTimeout (once)
    setTimeout( () => {
      ctx.clearRect(0,0,canvas.width, canvas.height);
      world.update();
      render();

      // the method takes a callback to be invoked before next re-render
      requestAnimationFrame(update);
    }, 1000 / fps);
  }

  render();
  update();
});