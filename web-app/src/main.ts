import {renderCanvas} from "./canvas/canvas.ts";
import {universe} from "./universe/universe.ts";

const renderLoop = () => {
    universe.tick();

    renderCanvas()

    setTimeout(() => {
        requestAnimationFrame(renderLoop);
    }, 25);
};

renderCanvas()
requestAnimationFrame(renderLoop);
