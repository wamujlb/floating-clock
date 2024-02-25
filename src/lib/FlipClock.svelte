<script lang="ts">
	import { roundToNearestEven } from '$lib/utils';
	import FlipItem from './FlipItem.svelte';
	import PomodoroFlipItem from './Pomodoro/PomodoroFlipItem.svelte';

	export let hours: number;
	export let minutes: number;
	export let seconds: number;
	export let clockSize: number;
	export let showSeconds: boolean;
	export let pomodoro: App.PomodoroSettings;

	$: gap = roundToNearestEven(clockSize);
</script>

<div
	class="flip-clock"
	style:gap="{gap}px"
	style:padding="{gap}px"
	class:show-seconds={showSeconds}
	class:show-third-col={showSeconds || pomodoro.showPomodoro}
>
	<div class="hours">
		<FlipItem value={hours} maxValue={23} delay={59 * 60 * 1000} />
	</div>

	<div class="minutes">
		<FlipItem value={minutes} maxValue={59} delay={59 * 1000} />
	</div>

	{#if showSeconds}
		<div class="seconds">
			<FlipItem value={seconds} maxValue={59} />
		</div>
	{/if}

	{#if pomodoro.showPomodoro}
		<div class="pomodoro">
			<PomodoroFlipItem interval={pomodoro.interval} focusTime={pomodoro.focusTime} />
		</div>
	{/if}
</div>

<style>
	.flip-clock {
		display: grid;
		height: 100vh;
		grid-template-columns: 2fr 2fr;
		grid-template-rows: 1fr 1fr;

		&.show-third-col {
			grid-template-columns: repeat(5, 1fr);

			.hours {
				grid-column-start: 1;
				grid-column-end: 3;
			}

			.minutes {
				grid-column-start: 3;
				grid-column-end: 5;
			}
		}

		&.show-seconds {
			.pomodoro {
				grid-row-start: 1;
				grid-row-end: 2;
			}
		}
	}

	.hours,
	.minutes {
		grid-row-start: 1;
		grid-row-end: 3;
	}

	.seconds {
		grid-row-start: 2;
		grid-row-end: 3;
	}

	.pomodoro {
		grid-row-start: 2;
		grid-row-end: 3;
	}
</style>
