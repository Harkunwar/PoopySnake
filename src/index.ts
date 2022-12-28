import init, { World, Direction, GameStatus } from "poopy_snake_wasm";
import { initFavicon } from "./favicon";

const { memory } = await init();
const CELL_SIZE = 40;
const WORLD_WIDTH = 12;
const WORLD_SIZE = WORLD_WIDTH * WORLD_WIDTH;
const SNAKE_SPAWN_INDEX = Date.now() % WORLD_SIZE;
const world = World.new(WORLD_WIDTH, SNAKE_SPAWN_INDEX);
const worldWidth = world.get_width();
const canvas = document.getElementById("snake-canvas") as HTMLCanvasElement;
const context = canvas.getContext("2d")!;
canvas.height = worldWidth * CELL_SIZE + 4 * CELL_SIZE;
canvas.width = worldWidth * CELL_SIZE;

canvas.addEventListener("click", function () {
  const status = world.get_game_status();
  if (status === undefined) {
    world.start_game();
    play();
  } else {
    location.reload();
  }
});

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

  snakeCells
    .filter((cellIndex, index) => !(index > 0 && cellIndex === snakeCells[0]))
    .forEach((cellIndex, index) => {
      const column = cellIndex % worldWidth;
      const row = Math.floor(cellIndex / worldWidth);

      context.fillStyle = index == 0 ? "#00A8CC" : "#0C7B93";

      context.beginPath();
      context.fillRect(
        column * CELL_SIZE,
        row * CELL_SIZE,
        CELL_SIZE,
        CELL_SIZE,
      );
      context.stroke();
    });
}

function drawEmoji(emoji, row, column) {
  context.beginPath();
  context.font = context.font.replace(/\d+px/, `${CELL_SIZE * 0.8}px`);
  context.fillText(
    emoji,
    column * CELL_SIZE + 5,
    row * CELL_SIZE + CELL_SIZE - 10,
    CELL_SIZE,
  );
  context.stroke();
}

function drawReward() {
  const rewardCellIndex = world.get_reward_cell();
  if (rewardCellIndex === undefined) {
    return;
  }
  const column = rewardCellIndex % worldWidth;
  const row = Math.floor(rewardCellIndex / worldWidth);
  drawEmoji("🐀", row, column);
}

function drawPoop() {
  const poopCellIndex = world.get_poop_cell();
  if (poopCellIndex === undefined) {
    return;
  }
  const column = poopCellIndex % worldWidth;
  const row = Math.floor(poopCellIndex / worldWidth);
  drawEmoji("💩", row, column);
}

function drawWelcomeScreen() {
  const center = (worldWidth * CELL_SIZE) / 2;

  // define the arc path
  context.beginPath();
  context.arc(center, center, 100, 0, 2 * Math.PI, false);

  // stroke it
  const originalLineWidth = context.lineWidth;
  context.lineWidth = 5;
  context.stroke();
  context.lineWidth = originalLineWidth;

  // make alpha solid (the color doesn't matter)
  context.fillStyle = "#ffffff";

  // change composite mode and fill
  context.globalCompositeOperation = "destination-out";
  context.fill();

  // reset composite mode to default
  context.globalCompositeOperation = "source-over";

  context.fillStyle = "#00A8CC";
  context.font = "25px sans-serif";

  let textString = "Poopy Snake",
    textWidth = context.measureText(textString).width;

  context.fillText(
    textString,
    canvas.width / 2 - textWidth / 2,
    canvas.width / 2 - 10,
  );

  context.font = "20px sans-serif";

  (textString = "Start Pooping!"),
    (textWidth = context.measureText(textString).width);

  context.fillText(
    textString,
    canvas.width / 2 - textWidth / 2,
    canvas.width / 2 + 20,
  );
}

function drawPoints() {
  const points = world.get_points();
  context.fillStyle = "#00A8CC";
  context.font = "25px sans-serif";

  const textString = `Points: ${points}`;
  const textWidth = context.measureText(textString).width;

  context.fillText(
    textString,
    canvas.width / 2 - textWidth / 2,
    canvas.height - 40,
  );
}

function paint() {
  drawWorld();
  drawSnake();
  drawReward();
  drawPoop();
  drawPoints();
}

function play() {
  const status = world.get_game_status();

  if (status === GameStatus.Won || status === GameStatus.Lost) {
    return;
  }

  const fps = 6;
  setTimeout(function () {
    context.clearRect(0, 0, canvas.width, canvas.height);
    world.step();
    paint();
    requestAnimationFrame(play);
  }, 1000 / fps);
}

paint();
drawWelcomeScreen();
initFavicon();
