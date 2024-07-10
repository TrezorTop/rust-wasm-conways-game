import { universe } from "../universe/universe.ts";
import { memory } from "wasm-pkg/web_assembly_binary_bg.wasm";

const CELL_SIZE = 5
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const canvas = document.querySelector<HTMLCanvasElement>("#canvas");

/**
 * Handles the click event on the canvas element. Calculates the row and column of the clicked cell based on the mouse coordinates,
 * and toggles the state of the cell in the universe.
 *
 * @param event - The click event object.
 */
if (!canvas) {
  throw Error("Canvas element not found");
}

canvas.onclick = (event) => {
  const boundingRect = canvas.getBoundingClientRect();

  const scaleX = canvas.width / boundingRect.width;
  const scaleY = canvas.height / boundingRect.height;

  const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
  const canvasTop = (event.clientY - boundingRect.top) * scaleY;

  const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), universe.height() - 1);
  const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), universe.width() - 1);

  universe.toggle_cell(row, col);

  renderCanvas();
}

const ctx = canvas.getContext('2d');

if (!ctx) {
  throw Error("Canvas context not found");
}

/**
 * Draws the grid lines on the canvas.
 * The grid lines are drawn based on the width and height of the universe, with each cell represented by a square of size `CELL_SIZE`.
 * The grid lines are drawn in the `GRID_COLOR`.
 */
const drawGrid = () => {
  const width = universe.width();
  const height = universe.height();

  canvas.height = (CELL_SIZE + 1) * height + 1;
  canvas.width = (CELL_SIZE + 1) * width + 1;

  ctx.beginPath();
  ctx.strokeStyle = GRID_COLOR;

  // Vertical lines.
  for (let i = 0; i <= width; i++) {
    ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
    ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
  }

  // Horizontal lines.
  for (let j = 0; j <= height; j++) {
    ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
    ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
  }

  ctx.stroke();
}

/**
 * Calculates the index of a cell in the universe's cells array based on its row and column coordinates.
 *
 * @param row - The row of the cell.
 * @param column - The column of the cell.
 * @param width - The width of the universe.
 * @returns The index of the cell in the universe's cells array.
 */
const getIndex = (row: number, column: number, width: number) => {
  return row * width + column;
};

/**
 * Checks if a bit is set in a Uint8Array at the given index.
 *
 * @param n - The index of the bit to check.
 * @param arr - The Uint8Array to check the bit in.
 * @returns `true` if the bit is set, `false` otherwise.
 */
const bitIsSet = (n: number, arr: Uint8Array) => {
  const byte = Math.floor(n / 8);
  const mask = 1 << (n % 8);
  return (arr[byte] & mask) === mask;
};

/**
 * Renders the cells on the canvas based on the state of the universe.
 *
 * This function first gets a pointer to the universe's cells, constructs a Uint8Array overlaying the cells buffer,
 * and then iterates over each cell in the universe. It draws a white or black rectangle for each cell depending on
 * whether the cell is dead or alive, respectively.
 */
const drawCells = () => {
  const width = universe.width();
  const height = universe.height();

  // To draw the cells, we get a pointer to the universe's cells, construct a Uint8Array overlaying the cells buffer,
  // iterate over each cell, and draw a white or black rectangle depending on whether the cell is dead or alive, respectively
  const cellsPtr = universe.cells();
  // width * height / 8 since we have a cell per bit rather than per byte
  const cells = new Uint8Array(memory.buffer, cellsPtr, width * height / 8);

  ctx.beginPath();

  // Alive cells
  ctx.fillStyle = ALIVE_COLOR;
  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const index = getIndex(row, col, width);

      if (!bitIsSet(index, cells)) {
        continue;
      }

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      )
    }
  }

  // Dead cells
  ctx.fillStyle = DEAD_COLOR;
  for (let row = 0; row < height; row++) {
    for (let col = 0; col < width; col++) {
      const index = getIndex(row, col, width);

      if (bitIsSet(index, cells)) {
        continue;
      }

      ctx.fillRect(
        col * (CELL_SIZE + 1) + 1,
        row * (CELL_SIZE + 1) + 1,
        CELL_SIZE,
        CELL_SIZE
      )
    }
  }

  ctx.stroke();
}

/**
 * Renders the canvas by drawing the grid and the cells.
 */
export const renderCanvas = () => {
  drawGrid();
  drawCells();
}