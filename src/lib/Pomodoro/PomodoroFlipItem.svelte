<script lang="ts">
	import src from '$lib/assets/pomodoro.png';
	import { writable } from 'svelte/store';
	import { Play, Square, ArrowUp, ArrowDown } from 'lucide-svelte';
	import { onDestroy } from 'svelte';
	import { type TimerState } from './timer.utils';
	import { scale } from 'svelte/transition';

	export let interval: number;
	export let focusTime: number;
	$: intervalDuration = interval * 60; // in seconds
	$: focusDuration = focusTime * 60; // in seconds
	$: breakDuration = intervalDuration - focusDuration; // in seconds

	let startedInterval: NodeJS.Timeout;
	const timerState = writable<TimerState>({
		started: false,
		tick: intervalDuration,
		phase: 'idle',
	});
	$: isFocusPhase = $timerState.phase === 'focus';
	$: isBreakPhase = $timerState.phase === 'break';
	$: animationDuration = isFocusPhase ? `${focusDuration + 1}s` : `${breakDuration}s`;

	const toggleTimer = () => {
		clearInterval(startedInterval);

		timerState.update((v) => ({
			started: !v.started,
			tick: intervalDuration,
			phase: !v.started ? 'focus' : 'idle',
		}));

		if (!$timerState.started) return;

		startedInterval = setInterval(() => {
			timerState.update((v) => {
				const tick = v.tick < intervalDuration ? v.tick + 1 : 0;
				const phase = tick < focusDuration ? 'focus' : 'break';
				return { ...v, tick, phase };
			});
		}, 1000);
	};

	onDestroy(() => {
		clearInterval(startedInterval);
	});
</script>

<div class="container">
	<div
		class="img absolute-center"
		style="background-image: url({src});"
		style:opacity={$timerState.started ? 0.4 : 1}
	/>

	{#if $timerState.started}
		<div
			style="background-image: url({src});"
			class="img img-mask absolute-center"
			class:animate-focus={isFocusPhase}
			class:animate-break={isBreakPhase}
			style:--animation-duration={animationDuration}
		/>

		<div
			class="phase-icon"
			class:phase-icon-focus={isFocusPhase}
			class:phase-icon-break={isBreakPhase}
			transition:scale={{ duration: 500, start: 1.4, opacity: 0 }}
		>
			{#if isFocusPhase}
				<ArrowDown color="#ff5861" size="100%" strokeWidth={2} />
			{:else}
				<ArrowUp color="#00aa6e" size="100%" strokeWidth={2} />
			{/if}
		</div>
	{/if}

	<button class="btn btn-square btn-ghost btn-play absolute-center" on:click={toggleTimer}>
		{#if $timerState.started}
			<Square size="70%" />
		{:else}
			<Play size="70%" />
		{/if}
	</button>
</div>

<style>
	.container {
		width: 100%;
		height: 100%;
		display: flex;
		background-color: #1f1f1f;
		justify-content: center;
		align-items: center;
		border-radius: 15%;
		overflow: hidden;
		position: relative;

		&:hover .btn-play {
			opacity: 1;
		}
	}

	.btn-play {
		opacity: 0;
		width: 100%;
		height: 100%;
		transition: opacity 0.3s;
	}

	.absolute-center {
		position: absolute;
		left: 50%;
		top: 50%;
		transform: translate(-50%, -50%);
	}

	.phase-icon {
		position: absolute;
		left: 5%;
		top: 5%;
		width: 20%;
		height: 20%;
	}

	.phase-icon-focus {
		animation: fadeDown 2s infinite;
	}

	.phase-icon-break {
		animation: fadeUp 2s infinite;
	}

	.img {
		width: 70%;
		height: 70%;
		background-size: cover;
		opacity: 0.4;
	}

	.img-mask {
		opacity: 1;
	}

	.animate-focus {
		animation: animateFocusTime var(--animation-duration) linear;
	}

	.animate-break {
		animation: animateBreakTime var(--animation-duration) linear;
	}

	@keyframes fadeUp {
		0% {
			opacity: 0;
			transform: translateY(60%) scale(0.8);
		}
		40% {
			opacity: 1;
		}
		100% {
			opacity: 0;
			transform: translateY(0%) scale(1);
		}
	}

	@keyframes fadeDown {
		0% {
			opacity: 0;
			transform: translateY(0%) scale(0.8);
		}
		40% {
			opacity: 1;
		}
		100% {
			opacity: 0;
			transform: translateY(60%) scale(1);
		}
	}

	@keyframes animateFocusTime {
		from {
			clip-path: fill-box rect(0% 100% 100% 0px);
		}
		to {
			clip-path: fill-box rect(100% 100% 100% 0px);
		}
	}

	@keyframes animateBreakTime {
		from {
			clip-path: fill-box rect(100% 100% 100% 0px);
		}
		to {
			clip-path: fill-box rect(0% 100% 100% 0px);
		}
	}
</style>
