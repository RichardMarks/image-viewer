<script lang="ts">
	import { goto } from '$app/navigation';
	import BrowseFile from '$lib/components/BrowseFile.svelte';

	let selectedFile: string = '';

	function handleShow() {
		const path = encodeURIComponent(selectedFile);
		const url = `/show?path=${path}`;
		goto(url);
	}
</script>

<div class="flex items-center justify-between p-4 h-full">
	<BrowseFile bind:selectedFile />
	<div>
		{#if selectedFile.length}
			<button
				on:click={handleShow}
				class=" ml-4 rounded-md bg-white/10 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-white/20"
				>Preview</button
			>
		{/if}
	</div>
</div>

<style lang="postcss">
	:global(html) {
		background-color: theme(colors.gray.900);
		color: theme(colors.white);
		height: 100%;
	}
	:global(body) {
		height: 100%;
	}
</style>
