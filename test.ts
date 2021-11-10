import { CanvasWindowApi } from "./mod.ts";

const canvas = CanvasWindowApi("Canvas Api");
const ctx = canvas.getContext();

for (let index = 0; index < 700; index += 1) {
    ctx.fillStyle = "#00000005";
    ctx.fillRect(0, 0, index, 100);
    if (index % 20 == 0) canvas.drawToWindow();
}