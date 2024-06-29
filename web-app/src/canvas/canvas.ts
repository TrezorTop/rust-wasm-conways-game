import {memory} from "../../wasm-pkg/rust_wasm_conways_game_bg.wasm";
import {universe} from "../universe/universe.ts";

const CELL_SIZE = 10; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const canvas = document.querySelector<HTMLCanvasElement>("#canvas");

if (!canvas) {
    throw Error("Canvas element not found");
}

const ctx = canvas.getContext('2d');

if (!ctx) {
    throw Error("Canvas context not found");
}

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

const getIndex = (row: number, column: number, width: number) => {
    return row * width + column;
};

const bitIsSet = (n: number, arr: Uint8Array) => {
    const byte = Math.floor(n / 8);
    const mask = 1 << (n % 8);
    return (arr[byte] & mask) === mask;
};

const drawCells = () => {
    const width = universe.width();
    const height = universe.height();

    // To draw the cells, we get a pointer to the universe's cells, construct a Uint8Array overlaying the cells buffer,
    // iterate over each cell, and draw a white or black rectangle depending on whether the cell is dead or alive, respectively
    const cellsPtr = universe.cells();
    // width * height / 8 since we have a cell per bit rather than per byte
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height / 8);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const index = getIndex(row, col, width);

            ctx.fillStyle = bitIsSet(index, cells) ? ALIVE_COLOR : DEAD_COLOR;

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

export const renderCanvas = () => {
    drawGrid();
    drawCells();
}