<script lang="ts">
	import { goto } from '$app/navigation';
	export let src: string = '';

	function handleReturnHome() {
		goto('/');
	}
</script>

{#if src.length}
	<div on:click={handleReturnHome} role="button" tabindex="0" on:keydown={handleReturnHome}>
		<img alt={src} {src} class="absolute inset-0 h-full w-full object-cover object-right" />
	</div>
{/if}
