<script lang="ts">
	import { roundToNearestEven } from '$lib/utils';
	import FlipItem from './FlipItem.svelte';
	import PomodoroFlipItem from '$lib/pomodoro/PomodoroFlipItem.svelte';

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
	class:show-pomodoro={pomodoro.showPomodoro}
>
	{#if pomodoro.showPomodoro}
		<div class="pomodoro">
			<PomodoroFlipItem interval={pomodoro.interval} focusTime={pomodoro.focusTime} />
		</div>
	{/if}

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
</div>

<style>
	.flip-clock {
		display: grid;
		height: 100vh;
		grid-template-columns: repeat(4, 1fr);
		grid-template-rows: 1fr 1fr;

		&:not(.show-pomodoro) {
			&.show-seconds {
				grid-template-columns: repeat(5, 1fr);
			}

			.hours {
				grid-column: 1 / 3;
			}

			.minutes {
				grid-column: 3 / 5;
			}
		}

		&.show-pomodoro {
			grid-template-columns: repeat(6, 1fr);

			&.show-seconds {
				grid-template-columns: repeat(7, 1fr);
			}

			.hours {
				grid-column: 3 / 5;
			}

			.minutes {
				grid-column: 5 / 7;
			}

			.pomodoro {
				grid-column: 1 / 3;
			}
		}
	}

	.hours,
	.minutes,
	.pomodoro {
		grid-row: 1 / 3;
	}

	.pomodoro {
		grid-column: 1 / 3;
	}

	.seconds {
		grid-row: 2 / 3;
	}
</style>
