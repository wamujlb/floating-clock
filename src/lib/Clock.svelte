<script lang="ts">
	import { window as tauriWindow } from '@tauri-apps/api';
	import { listen } from '@tauri-apps/api/event';
	import { onDestroy, onMount } from 'svelte';
	import { initSettings } from '$lib/utils';

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

	const formatDigit = (digit: number): string => {
		return digit < 10 ? `0${digit}` : digit.toString();
	};
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<container
	data-tauri-drag-region
	on:mousedown={handleContainerMousedown}
	style:opacity={settings.opacity}
	class={settings.showSeconds ? 'show-seconds' : ''}
>
	<div class="digit-wrapper">
		<div class="digit">{formatDigit(hours)}</div>
	</div>

	<div class="digit-wrapper">
		<div class="digit">{formatDigit(minutes)}</div>
	</div>

	{#if settings?.showSeconds}
		<div class="digit-wrapper second">
			<div class="digit">{formatDigit(seconds)}</div>
		</div>
	{/if}
</container>

<style>
	container {
		display: grid;
		height: 100vh;
		grid-template-columns: 100vh 100vh;
		grid-template-rows: 100vh;
		justify-content: space-between;

		&.show-seconds {
			grid-template-columns: 100vh 100vh 50vh;
		}
	}

	.digit-wrapper {
		height: 100%;
		width: 100%;
		font-size: 3rem;
		display: flex;
		align-items: center;
		justify-content: center;
		background-color: black;
		color: white;
		border-radius: 10%;
		container-type: size;
		overflow: hidden;
		user-select: none;
		-webkit-user-select: none;
		cursor: default;
	}

	.digit {
		font-size: 75cqw;
		font-weight: 600;
		font-family: Arial, Helvetica, sans-serif;
	}

	.second {
		height: 50vh;
		place-self: end;
	}
</style>
