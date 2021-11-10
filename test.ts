import { init, create_window as createWindow, push_png_buffer as pushPngBuffer, quick_wait as quickWait, update_loop as updateLoop, show_window as showWindow } from "./bindings/bindings.ts";

import { createCanvas } from "https://deno.land/x/canvas/mod.ts";
// const _lib = Deno.dlopen("./target/debug/libtest.dylib", {
export function CanvasWindowApi(title = "Canvas Api", width = 700, height = 400) {
    init();
    createWindow({ label: title, resizeable: false, width, height });
    showWindow();
    setInterval(() => {
        if (updateLoop()) Deno.exit(0);
    }, 20);
    const canvas = createCanvas(width, height);
    const ctx = canvas.getContext("2d");
    updateLoop();
    return {
        ...ctx,
        drawToWindow: () => {
            console.time();
            pushPngBuffer(canvas.toBuffer("image/png"));
            console.timeEnd();
            quickWait();
        }
    };
}


const ctx = CanvasWindowApi("Canvas Api");

console.time("completeDraw")
for (let index = 0; index < 700; index += 1) {
    ctx.fillStyle = "#FFFFFFFF";
    ctx.fillRect(0, 0, index, 100);
    if (index % 20 == 0) ctx.drawToWindow();
}
console.timeEnd("completeDraw")