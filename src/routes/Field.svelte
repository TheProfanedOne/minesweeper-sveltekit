<script lang="ts">
    import { gameState } from "./stores";
    import { open, toggleFlag, boardState, isFinished } from "minesweeper-sveltekit-wasm";

    export let infoCode: number;
    export let pos: [number, number];

    enum FieldState {
        Closed,
        Flag,
        Open,
    }

    interface FieldInfo {
        state: FieldState,
        mines: number,
        contents: string,
    }

    function parseInfoCode(code: number): FieldInfo {
        let state = Math.floor(code / 10);
        let mines = code % 10;
        
        return state === 0 ? {
            state: FieldState.Closed,
            mines,
            contents: '',
        } : state === 1 ? {
            state: FieldState.Closed,
            mines,
            contents: '🚩',
        } : {
            state: FieldState.Open,
            mines,
            contents: mines === 9 ? '💣' : mines === 0 ? '' : String(mines),
        };
    }

    let fieldInfo = parseInfoCode(infoCode);

    const handleLeftClick = (evt: MouseEvent) => {
        evt.preventDefault();

        if (isFinished() !== 0 || fieldInfo.state === FieldState.Flag) return;

        const [x, y] = pos;
        open(x, y);

        if ($gameState !== boardState()) gameState.reload();
    };

    const handleRightClick = (evt: MouseEvent) => {
        evt.preventDefault();

        if (isFinished() !== 0 || fieldInfo.state === FieldState.Open) return;

        const [x, y] = pos;
        toggleFlag(x, y);

        gameState.reload();
    };

    const FIELD_SIDE = 50;

    let finished = isFinished();

    const gray  = fieldInfo.state in [FieldState.Closed, FieldState.Flag] && finished === 0;
    const mine  = fieldInfo.state === FieldState.Open && fieldInfo.mines === 9 && finished === 1;
    const flag  = fieldInfo.state === FieldState.Flag && finished === 2;

    const one   = fieldInfo.mines === 1;
    const two   = fieldInfo.mines === 2;
    const three = fieldInfo.mines === 3;
    const four  = fieldInfo.mines === 4;
    const five  = fieldInfo.mines === 5;
    const six   = fieldInfo.mines === 6;
    const seven = fieldInfo.mines === 7;
    const eight = fieldInfo.mines === 8;
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div
    style="--field-side: {FIELD_SIDE}px;"
    class:gray class:mine class:flag
    class:one class:two class:three class:four
    class:five class:six class:seven class:eight
    on:click={handleLeftClick}
    on:contextmenu={handleRightClick}
>
    {fieldInfo.contents}
</div>

<style>
    div {
        box-sizing: border-box;
        cursor: pointer;
        display: inline;
        text-decoration: none;
        width: var(--field-side) !important;
        height: var(--field-side) !important;
        line-height: 2.4rem;
        padding-top: 0.25rem;
        padding-bottom: 0.15rem;
        text-align: center;
        border: 2px solid black;
    }

    .gray  { background-color: #333333; }
    .mine  { background-color: crimson; }
    .flag  { background-color: limegreen; }

    .one   { color: blue; }
    .two   { color: green; }
    .three { color: red; }
    .four  { color: purple; }
    .five  { color: maroon; }
    .six   { color: rgb(43, 168, 156); }
    .seven { color: black; }
    .eight { color: #585858; }
</style>
