<script lang="ts">
    import GridTile from "$lib/GridTile";
    export let grid: GridTile[][];
    import { invoke } from "@tauri-apps/api";

    type Beano = [{ x: number; y: number }, { task: string }];
    type Spawner = [{ x: number; y: number }, Beano[]][];

    let spawner: Spawner | null = null;
    let beanoz: Beano[] = [];

    let canvas: HTMLCanvasElement;

    $: ctx = canvas?.getContext("2d");

    invoke("generate_random", { grid }).then((f: Spawner) => {
        spawner = f;
    });

    $: if (spawner != null) {
        setTimeout(iterate, 100);
    }

    async function iterate() {
        [beanoz, spawner] = await invoke("run_iteration", { beanoz, spawner });
        if (beanoz.find((beano) => beano[1].task !== "Walking")) {
            setTimeout(iterate, 100);
        }
    }

    $: if (ctx) {
        for (const x in grid) {
            for (const y in grid[x]) {
                const tile = grid[x][y];

                switch (tile) {
                    case GridTile.Entry:
                        ctx.fillStyle = "rgb(0, 255, 0)";
                        break;
                    case GridTile.None:
                        ctx.fillStyle = "rgb(0, 0, 0)";
                        break;
                    case GridTile.Seat:
                        ctx.fillStyle = "rgb(150, 150, 150)";
                        break;
                    case GridTile.Walkway:
                        ctx.fillStyle = "rgb(255,255,255)";
                        break;
                }
                

                ctx.fillRect(parseInt(x) * 28, parseInt(y) * 28, 25, 25);
            
            }
        }
    }
    
    $: if (ctx) {
        for (const beano of beanoz) {
            ctx.fillStyle = "rgb(255,255,0)";
            ctx.fillRect(beano[0].x * 28 + 5, beano[0].y * 28 + 5, 15, 15);
        }
    }
</script>

<canvas
    bind:this={canvas}
    width={grid.length * 28}
    height={grid[0].length * 28}
/>

<style lang="scss">
</style>
