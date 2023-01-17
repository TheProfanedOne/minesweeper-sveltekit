<script lang="ts">
    import init, {
        fieldsOpened, flagsRemaining, isFinished,
        width, height
    } from 'minesweeper-sveltekit-wasm';
	import { onMount } from 'svelte';
    import Field from './Field.svelte';
    import Subtitles from './Subtitles.svelte';
    import { gameState } from './stores';

    let initialised = false;

    onMount(() => init().then(() => {
        gameState.reload();
        initialised = true;
    }));
</script>

<svelte:head>
    <title>Minesweeper</title>
    <meta name=description content="Minesweeper web app using sveltekit and wasm">
    <style>
        body {
            font-size: 250%;
            background-color: rgba(0, 47, 150, 0.637);
        }
    </style>
</svelte:head>

<main>
    {#if initialised}
        {@const fieldSide = 50}
        {@const padding   = 32}

        {@const sWidth  = width()}
        {@const sHeight = height()}
        {@const tWidth  = (sWidth * fieldSide) + padding}
        {@const tHeight = (sHeight * fieldSide) + padding}
        <section style="--width: {sWidth}; --height: {sHeight}; --t-width: {tWidth}px; --t-height: {tHeight}px;">
            {#each $gameState as row, y}
                {#each row as infoCode, x}
                    <Field {infoCode} pos={[x, y]} />
                {/each}
            {/each}
        </section>
        <Subtitles fin={isFinished()} fo={fieldsOpened()} fr={flagsRemaining()} />
    {/if}
</main>

<style>
    main {
        font-family: monospace;
        font-weight: 900;
        display: flex;
        justify-content: center;
        justify-self: center;
        flex-flow: column wrap;
        align-items: center;
    }

    section {
        background-color: darkgray;
        padding: 1rem;
        box-sizing: border-box;
        display: inline-grid;
        width: var(--t-width);
        height: var(--t-height);
        grid-template: repeat(var(--height), auto) / repeat(var(--width), auto);
    }
</style>
