import init, { World, Direction } from "poopy_snake_wasm";

const { memory } = await init();
const CELL_SIZE = 40;
const WORLD_WIDTH = 8;
const SNAKE_SPAWN_INDEX = Date.now() % (WORLD_WIDTH * WORLD_WIDTH);
const world = World.new(WORLD_WIDTH, SNAKE_SPAWN_INDEX);
const worldWidth = world.get_width();
const canvas = document.querySelector("canvas")!;
const context = canvas.getContext("2d")!;
canvas.height = worldWidth * CELL_SIZE;
canvas.width = worldWidth * CELL_SIZE;

document.addEventListener("keydown", function (event) {
  switch (event.code) {
    case "ArrowLeft":
      world.set_snake_direction(Direction.Left);
      break;
    case "ArrowRight":
      world.set_snake_direction(Direction.Right);
      break;
    case "ArrowUp":
      world.set_snake_direction(Direction.Up);
      break;
    case "ArrowDown":
      world.set_snake_direction(Direction.Down);
      break;
  }
});

function drawWorld() {
  context.beginPath();

  for (let x = 0; x < worldWidth + 1; x++) {
    context.moveTo(CELL_SIZE * x, 0);
    context.lineTo(CELL_SIZE * x, worldWidth * CELL_SIZE);
  }

  for (let y = 0; y < worldWidth + 1; y++) {
    context.moveTo(0, CELL_SIZE * y);
    context.lineTo(worldWidth * CELL_SIZE, CELL_SIZE * y);
  }

  context.stroke();
}

function drawSnake() {
  const snakeCellPointer = world.get_snake_cell_pointer();
  const snakeLength = world.get_snake_length();
  const snakeCells = new Uint32Array(
    memory.buffer,
    snakeCellPointer,
    snakeLength,
  );

  snakeCells.forEach((cellIndex, index) => {
    const column = cellIndex % worldWidth;
    const row = Math.floor(cellIndex / worldWidth);

    context.fillStyle = index == 0 ? "#7878db" : "#000000";

    context.beginPath();
    context.fillRect(column * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
    context.stroke();
  });
}

function paint() {
  drawWorld();
  drawSnake();
}

function update() {
  const fps = 10;
  setTimeout(function () {
    context.clearRect(0, 0, canvas?.width ?? 0, canvas?.height ?? 0);
    world.step();
    paint();
    requestAnimationFrame(update);
  }, 1000 / fps);
}

paint();
update();
