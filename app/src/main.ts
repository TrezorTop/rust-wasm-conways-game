import { renderCanvas } from "./canvas/canvas.ts";
import { universe } from "./universe/universe.ts";
import { fps } from "./fps/fps.ts";


const clearButton = document.querySelector<HTMLButtonElement>('#clear');
const pauseButton = document.querySelector<HTMLButtonElement>('#pause');
const resetButton = document.querySelector<HTMLButtonElement>('#reset');
const rangeInput = document.querySelector<HTMLInputElement>('#speed');

let animationId: number | null = null;
let timeoutId: number | null = null;
let timeout = 125;

if (pauseButton) {
  pauseButton.onclick = () => {
    if (animationId === null) {
      animationId = requestAnimationFrame(renderLoop);
      pauseButton.textContent = 'Pause';
    } else {
      cancelAnimationFrame(animationId);
      animationId = null;
      pauseButton.textContent = 'Play';
    }
  }
}

if (rangeInput) {
  rangeInput.onchange = (event) => {
    const target = event.target as HTMLInputElement;
    timeout = parseInt(target.value);

    if (timeoutId) {
      clearTimeout(timeoutId);
    }

    if (animationId !== null) {
      timeoutId = setTimeout(() => {
        animationId = requestAnimationFrame(renderLoop);
      }, timeout);
    }
  }
}

if (resetButton) {
  resetButton.onclick = () => {
    universe.reset();
    renderCanvas();
  }
}

if (clearButton) {
  clearButton.onclick = () => {
    universe.clear();
    renderCanvas();
  }
}

/**
 * Requests the next frame of the animation loop.
 * If an animation ID is currently set, it will request the next frame using `requestAnimationFrame`.
 * Otherwise, it will not schedule the next frame.
 */
const requestFrame = () => {
  if (!animationId) return;

  animationId = requestAnimationFrame(renderLoop);
}

/**
 * The main render loop for the application.
 * This function is responsible for updating the game state, rendering the canvas, and scheduling the next frame.
 * If a timeout is set, it will use setTimeout to schedule the next frame after the timeout expires.
 * Otherwise, it will use requestAnimationFrame to schedule the next frame.
 */
const renderLoop = () => {
  fps.render();
  if (timeoutId) clearTimeout(timeoutId);

  universe.tick();

  renderCanvas()

  if (timeout) {
    timeoutId = setTimeout(() => {
      requestFrame()
    }, timeout);
  } else {
    requestFrame();
  }

};

fps.render();
renderCanvas()
