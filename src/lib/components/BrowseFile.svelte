<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';

	export let selectedFile: string = '';

	function browseForFile() {
		invoke('browse_for_file')
			.then((filePath: unknown) => {
				selectedFile = filePath as string;
			})
			.catch((_err: unknown) => {
				selectedFile = '';
			});
	}
</script>

<div class="flex flex-row w-full">
	<input
		type="text"
		bind:value={selectedFile}
		readonly
		class="block w-full rounded-md border-0 px-2 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 placeholder:text-gray-400 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:text-sm sm:leading-6"
		placeholder="Browse for an image"
	/>

	<button
		on:click={browseForFile}
		type="button"
		class="ml-4 rounded-md bg-indigo-900 px-3.5 py-2.5 text-sm font-semibold text-white shadow-sm hover:bg-indigo-400 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-500"
		>Browse</button
	>
</div>
