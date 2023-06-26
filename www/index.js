import { Universe } from "../pkg/orochimaru";
import { memory } from "../pkg/orochimaru_bg"
const WIDTH = 20;
const HEIGHT = 20;
const universe = Universe.new(WIDTH, HEIGHT);

const CELL_SIZE = 20; // px
const GRID_COLOR = "#CCCCCC";
const FOOD_COLOR = "#FFFFFF";
const SNAKE_COLOR = "#000000";
const EMPTY_COLOR = "#FFFFFF";

// Give the canvas room for all of our cells and a 1px border
// around each of them.
const canvas = document.getElementById("orochimaru-canvas");
canvas.height = ((CELL_SIZE + 1) * HEIGHT) + 1;
canvas.width = ((CELL_SIZE + 1) * WIDTH) + 1;

//WTF is this ?
const ctx = canvas.getContext('2d');

const drawGrid = () => {
  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let i = 0; i <= WIDTH; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1), 0);
    ctx.lineTo(i * (CELL_SIZE + 1), (CELL_SIZE + 1) * HEIGHT);
    ctx.stroke();
  }

  // Horizontal lines.
  for (let j = 0; j <= HEIGHT; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1));
    ctx.lineTo((CELL_SIZE + 1) * WIDTH, j * (CELL_SIZE + 1));
    ctx.stroke();
  }

};

const paintCell = (r, c, color) => {
  ctx.fillStyle = color;
  ctx.fillRect(
    r * (CELL_SIZE + 1) + 1,
    c * (CELL_SIZE + 1) + 1,
    CELL_SIZE,
    CELL_SIZE
  );
  ctx.stroke();
}

const paintSnake = (color) => {
  const snakeLen = universe.get_snake_len();
  const snakePtr = universe.get_snake_position();
  const snakeCells = new Uint32Array(memory.buffer, snakePtr, snakeLen * 2);
  console.log(snakeCells.length);
  for (let i = 0; i < snakeLen; i++) {
    let r = snakeCells[i * 2];
    let c = snakeCells[i * 2 + 1];
    paintCell(r, c, color);
  }
}

const renderLoop = () => {
  paintSnake(EMPTY_COLOR);
  universe.tick();
  paintSnake(SNAKE_COLOR);
  requestAnimationFrame(renderLoop);
};

drawGrid();
paintSnake(SNAKE_COLOR);
requestAnimationFrame(renderLoop);
