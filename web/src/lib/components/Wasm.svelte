<script lang="ts">
	import { onMount } from 'svelte';
	import init from '../../../../wasm/pkg';

	onMount(() => {
		init();
	});
</script>

<div id="wasm-canvas-container"></div>

<style>
	div {
		position: fixed;
		top: 0;
		left: 0;
		width: 100vw;
		height: 100vh;
		z-index: -100;
	}
</style>
