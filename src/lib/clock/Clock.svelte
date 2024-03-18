<script lang="ts">
	import { window as tauriWindow } from '@tauri-apps/api';
	import { listen } from '@tauri-apps/api/event';
	import { onDestroy, onMount } from 'svelte';
	import FlipClock from '$lib/flip-clock/FlipClock.svelte';
	import { deserializeSettings, type SettingsPayload } from '$lib/settings/utils';
	import { writable } from 'svelte/store';
	import BinaryAnalogClock from '$lib/binary-analog-clock/binary-analog-clock.svelte';

	export let data: App.Settings;

	let settings = writable<App.Settings>(data);

	let settingsChangeListener: () => void;

	onMount(async () => {
		settingsChangeListener = await listen<SettingsPayload>('settings-change', ({ payload }) =>
			settings.set(deserializeSettings(payload))
		);
	});

	onDestroy(() => {
		// Destroy the listener when the component is destroyed
		settingsChangeListener();
	});

	const handleContainerMousedown = async (e: MouseEvent) => {
		await tauriWindow.appWindow.startDragging();
	};

	const date = new Date();
	let hours = date.getHours();
	let minutes = date.getMinutes();
	let seconds = date.getSeconds();

	setInterval(() => {
		const date = new Date();
		hours = date.getHours();
		minutes = date.getMinutes();
		seconds = date.getSeconds();
	}, 1000);

	$: clockVariant = $settings.variant;
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
	data-tauri-drag-region
	style:opacity={$settings.opacity}
	on:mousedown={handleContainerMousedown}
>
	{#if clockVariant === 'Flip'}
		<FlipClock
			{hours}
			{minutes}
			{seconds}
			showSeconds={$settings.showSeconds}
			pomodoro={$settings.pomodoro}
			clockSize={$settings.clockSize}
		/>
	{/if}

	{#if clockVariant === 'BinaryAnalog'}
		<BinaryAnalogClock
			{hours}
			{minutes}
			{seconds}
			showSeconds={$settings.showSeconds}
			pomodoro={$settings.pomodoro}
			clockSize={$settings.clockSize}
		/>
	{/if}
</div>
