<script lang="ts">
	import { window as tauriWindow } from '@tauri-apps/api';
	import { listen } from '@tauri-apps/api/event';
	import { onDestroy, onMount } from 'svelte';
	import { initSettings } from '$lib/utils';
	import FlipClock from './FlipClock.svelte';

	let settingsChangeListener: () => void;
	let settings = initSettings;

	onMount(async () => {
		settingsChangeListener = await listen<App.SettingsChangePayload>(
			'settings-change',
			({ payload }) => {
				settings = {
					showSeconds: payload.show_seconds,
					opacity: payload.opacity,
					clockSize: payload.clock_size,
					variant: payload.variant,
					pomodoro: {
						showPomodoro: payload.pomodoro.show_pomodoro,
						interval: payload.pomodoro.interval,
						focusTime: payload.pomodoro.focus_time,
					},
				};
			}
		);
	});

	onDestroy(() => {
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
		showPomodoro={settings.pomodoro.showPomodoro}
		clockSize={settings.clockSize}
	/>
</div>
