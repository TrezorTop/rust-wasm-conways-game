import {renderCanvas} from "./canvas/canvas.ts";
import {universe} from "./universe/universe.ts";

const renderLoop = () => {
    universe.tick();

    renderCanvas()
    
    requestAnimationFrame(renderLoop);
};

renderCanvas()
requestAnimationFrame(renderLoop);
