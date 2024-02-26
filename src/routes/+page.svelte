<script lang="ts">
	import Clock from '$lib/clock/Clock.svelte';

	export let data: { data: App.Settings };
</script>

<main>
	<Clock data={data.data} />
</main>

<style>
	:global(html) {
		background-color: transparent !important;
	}
</style>
