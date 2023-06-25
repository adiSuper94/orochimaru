import { Universe } from "../pkg/orochimaru";
const WIDTH = 20;
const HEIGHT = 20;
const universe = Universe.new(WIDTH, HEIGHT);

const CELL_SIZE = 20; // px
const GRID_COLOR = "#CCCCCC";
const FOOD_COLOR = "#FFFFFF";
const SNAKE_COLOR = "#000000";

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
    c * (CELL_SIZE + 1) + 1,
    r * (CELL_SIZE + 1) + 1,
    CELL_SIZE,
    CELL_SIZE
  );
  ctx.stroke();
}


const renderLoop = () => {
  universe.tick();

  drawGrid();

  requestAnimationFrame(renderLoop);
};

drawGrid();
paintCell(0, 0);
requestAnimationFrame(renderLoop);
