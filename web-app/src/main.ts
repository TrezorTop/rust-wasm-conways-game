import {renderCanvas} from "./canvas/canvas.ts";
import {universe} from "./universe/universe.ts";


const clearButton = document.querySelector<HTMLButtonElement>('#clear');
const pauseButton = document.querySelector<HTMLButtonElement>('#pause');
const resetButton = document.querySelector<HTMLButtonElement>('#reset');
const rangeInput = document.querySelector<HTMLInputElement>('#speed');

let animationId: number | null = null;
let timeoutId: number | null = null;
let timeout = 250;

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


const renderLoop = () => {
    if (timeoutId) clearTimeout(timeoutId);

    universe.tick();

    renderCanvas()

    timeoutId = setTimeout(() => {
        if (!animationId) return;

        animationId = requestAnimationFrame(renderLoop);
    }, timeout);
};

renderCanvas()
