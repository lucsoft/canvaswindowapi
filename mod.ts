// Sometimes the cache doesn't update so replace _lib with const _lib = Deno.dlopen("./target/debug/libtest.dylib", {
import { init, create_window as createWindow, push_png_buffer as pushPngBuffer, quick_wait as quickWait, update_loop as updateLoop, show_window as showWindow } from "./bindings/bindings.ts";
import { createCanvas } from "https://deno.land/x/canvas/mod.ts";

export function CanvasWindowApi(title = "Canvas Api", width = 700, height = 400) {
    init();
    createWindow({ label: title, resizeable: false, width, height });
    showWindow();
    setInterval(() => {
        if (updateLoop()) Deno.exit(0);
    }, 20);
    const canvas = createCanvas(width, height);

    updateLoop();
    return {
        getContext: () => canvas.getContext("2d"),
        drawToWindow: () => {
            pushPngBuffer(canvas.toBuffer("image/png"));
            quickWait();
        }
    };
}
