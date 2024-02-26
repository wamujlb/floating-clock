<script lang="ts">
	import Clock from '$lib/Clock.svelte';

	export let data: { data: App.Settings };
</script>

<main>
	<Clock settings={data.data} />
</main>

<style>
	:global(html) {
		background-color: transparent !important;
	}
</style>
