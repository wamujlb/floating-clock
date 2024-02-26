<script lang="ts">
	import { window as tauriWindow } from '@tauri-apps/api';
	import { listen } from '@tauri-apps/api/event';
	import { onDestroy, onMount } from 'svelte';
	import FlipClock from '$lib/flip-clock/FlipClock.svelte';
	import { deserializeSettings, type SettingsPayload } from '$lib/settings/utils';

	export let settings: App.Settings;

	let settingsChangeListener: () => void;

	onMount(async () => {
		settingsChangeListener = await listen<SettingsPayload>('settings-change', ({ payload }) =>
			deserializeSettings(payload)
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
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
	data-tauri-drag-region
	style:opacity={settings.opacity}
	on:mousedown={handleContainerMousedown}
>
	<FlipClock
		{hours}
		{minutes}
		{seconds}
		showSeconds={settings.showSeconds}
		pomodoro={settings.pomodoro}
		clockSize={settings.clockSize}
	/>
</div>
