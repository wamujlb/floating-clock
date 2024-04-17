<script lang="ts">
	import { window as tauriWindow } from '@tauri-apps/api';
	import { listen } from '@tauri-apps/api/event';
	import { onDestroy, onMount } from 'svelte';
	import FlipClock from '$lib/flip-clock/FlipClock.svelte';
	import { deserializeSettings, type SettingsPayload } from '$lib/settings/utils';
	import { writable } from 'svelte/store';
	import BinaryAnalogClock from '$lib/binary-analog-clock/binary-analog-clock.svelte';

	export let data: App.Settings;

	type Time = {
		hours: number;
		minutes: number;
		seconds: number;
	};

	const getTime = (): Time => {
		const date = new Date();
		return {
			hours: date.getHours(),
			minutes: date.getMinutes(),
			seconds: date.getSeconds(),
		};
	};

	const time = writable<Time>(getTime());
	const settings = writable<App.Settings>(data);
	let intervalListener: number;
	let settingsChangeListener: () => void;

	onMount(async () => {
		settingsChangeListener = await listen<SettingsPayload>('settings-change', ({ payload }) =>
			settings.set(deserializeSettings(payload))
		);
	});

	onDestroy(() => {
		// Destroy the listener when the component is destroyed
		settingsChangeListener();
		clearInterval(intervalListener);
	});

	settings.subscribe(({ showSeconds }) => {
		const intervalTime = showSeconds ? 1000 : 60000;
		clearInterval(intervalListener);
		time.set(getTime());

		intervalListener = setInterval(() => {
			time.set(getTime());
		}, intervalTime);
	});

	const handleContainerMousedown = async (e: MouseEvent) => {
		await tauriWindow.appWindow.startDragging();
	};

	$: clockVariant = $settings.variant;
	$: hours = $time.hours;
	$: minutes = $time.minutes;
	$: seconds = $time.seconds;
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
