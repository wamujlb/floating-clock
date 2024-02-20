<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { fade } from 'svelte/transition';
	import { listen } from '@tauri-apps/api/event';
	import Clock from '$lib/Clock.svelte';
	import SettingsBtn from '$lib/SettingsBtn.svelte';
	import { writable } from 'svelte/store';

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

	let isHovered = writable(false);

	$: console.log($isHovered);
</script>

<main
	class="relative"
	on:mouseenter={() => isHovered.set(true)}
	on:mouseleave={() => isHovered.set(false)}
>
	{#if $isHovered}
		<div transition:fade={{ delay: 100, duration: 300 }}>
			<SettingsBtn />
		</div>
	{/if}

	<Clock />
</main>

<style>
	:global(html) {
		background-color: transparent !important;
	}
</style>
