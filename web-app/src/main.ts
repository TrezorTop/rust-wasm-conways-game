import {Universe} from "wasm-pkg"

const pre = document.getElementById("game-of-life-canvas");
const universe = Universe.new();

if (!pre) {
    throw new Error("Failed to find root element");
}

const renderLoop = () => {
    pre.textContent = universe.render();
    universe.tick();

    setTimeout(() => {
        requestAnimationFrame(renderLoop);
    }, 50);
};

requestAnimationFrame(renderLoop);