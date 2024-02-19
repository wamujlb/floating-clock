<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { emit, listen } from '@tauri-apps/api/event';
	import Clock from '$lib/Clock.svelte';
	import SettingsBtn from '$lib/SettingsBtn.svelte';

	type WindowPosition = {
		x: number;
		y: number;
	};

	let windowMoveListener: () => void;

	const windowPos: WindowPosition = {
		x: 0,
		y: 0,
	};

	onMount(async () => {
		windowMoveListener = await listen<WindowPosition>('tauri://move', (e) => {
			windowPos.x = e.payload.x;
			windowPos.y = e.payload.y;
		});
	});

	onDestroy(() => {
		windowMoveListener();
	});
</script>

<!-- <slot /> -->

<main>
	<SettingsBtn />

	<Clock />
</main>

<style>
	:global(html) {
		background-color: transparent !important;
	}
</style>
