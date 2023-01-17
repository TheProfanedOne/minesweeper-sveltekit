<script lang="ts">
    export let fin: number;
    export let fo: number;
    export let fr: number;
</script>

<span>
    {#if fin === 0}
        Fields Opened: {fo} | Flags Remaining: {fr}
    {:else if fin === 1}
        You lost. Refresh this page to try again.
    {:else}
        You won! Refresh this page to play again.
    {/if}
</span>

<style>
    span { font-size: 0.9em; }
</style>
