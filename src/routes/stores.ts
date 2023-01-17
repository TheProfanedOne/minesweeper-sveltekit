import { writable } from "svelte/store";
import { boardState } from "minesweeper-sveltekit-wasm";

function createGameState() {
    const { subscribe, set } = writable([] as Uint8Array[]);

    return {
        subscribe,
        reload: () => set(boardState())
    };
}

export const gameState = createGameState();
