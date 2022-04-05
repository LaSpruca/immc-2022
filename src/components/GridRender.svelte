<script lang="ts">
    import type GridTile from "$lib/GridTile";
    export let grid: GridTile[][];
    console.log(grid);
</script>

<div class="plane">
    {#each grid as column}
        <div class="column">
            {#each column as tile}
                <div class={`tile ${tile}`} />
            {/each}
        </div>
    {/each}
</div>

<style lang="scss">
    .plane {
        display: flex;
        flex-direction: row;
    }

    .column {
        display: flex;
        flex-direction: column;
    }

    .tile {
        width: 25px;
        height: 25px;

        &.None {
            background-color: black;
        }

        &.Walkway {
            background-color: white;
        }

        &.Seat {
            background-color: gray;
        }

        &.Entry {
            background-color: green;
        }
    }
</style>
