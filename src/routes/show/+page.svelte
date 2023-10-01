<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import ImageView from '$lib/components/ImageView.svelte';
	import { convertFileSrc } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';

	let selectedFilePath: string = '';

	onMount(() => {
		function gather(): Record<string, string> {
			const q: Record<string, string> = {};
			for (const [k, v] of new URL($page.url).searchParams.entries()) {
				q[k] = v;
			}
			return q;
		}
		selectedFilePath = convertFileSrc(decodeURIComponent(gather().path));
	});
</script>

{#if selectedFilePath.length}
	<ImageView src={selectedFilePath} />
{/if}
