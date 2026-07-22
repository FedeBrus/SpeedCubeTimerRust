<script lang="ts">
import { invoke } from "@tauri-apps/api/core";

let running = false;
let displayTime = 0;

let start: number = 0;

async function handleSpace(event: KeyboardEvent) {
    if (event.code !== "Space") return;

    event.preventDefault();

    if (!running) {
        await invoke("start_timer");
        running = true;
        displayTime = 0;
        start = performance.now();
        requestAnimationFrame(update);
    } else {
        const time = await invoke<number>("stop_timer");
        running = false;
        displayTime = time / 1000;
    }
}

function update() {
    if (!running) return;
    displayTime = Math.round((performance.now() - start) * 100) / 100 / 1000;
    requestAnimationFrame(update);
}

window.addEventListener(
    "keydown",
    handleSpace
);
</script>

<h1>
    {displayTime}
</h1>
