<script lang="ts">
	import type GridTile from "$lib/GridTile";

	import { invoke } from "@tauri-apps/api";
	import GridRender from "./components/GridRender.svelte";

	let grid: null | GridTile[][];

	const loadImage = async () => {
		invoke("load_image").then((_grid: GridTile[][]) => (grid = _grid));
	};
</script>

<main>
	{#if grid == null}
		<button on:click={loadImage}>Make da plane</button>
	{:else}
		<GridRender {grid} />
	{/if}
</main>

<style lang="scss">
	main {
		display: flex;
		justify-content: center;
		align-items: center;
		max-width: 100%;
		width: 100vw;
		height: 100vh;
	}
</style>
