<script lang="ts">
	import type { ChangeEventHandler } from 'svelte/elements';
	import { scale } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
	import { getInputValue } from '$lib/utils';
	import { writable } from 'svelte/store';

	export let onSettingsChange: (values: App.PomodoroSettings) => void;
	export let values: {
		showPomodoro: boolean;
		interval: number;
		focusTime: number;
	};

	$: breakTime = values.interval - values.focusTime;

	const MIN_INTERVAL = 5;
	const interval = writable(values.interval);
	interval.subscribe((v) => {
		onSettingsChange({
			...values,
			interval: v,
			focusTime: Math.min(values.focusTime, v),
		});
	});
	const handleIncrease = () => interval.update((v) => v + 5);
	const handleDecrease = () => interval.update((v) => Math.max(v - MIN_INTERVAL, MIN_INTERVAL));

	const handleChange: ChangeEventHandler<HTMLInputElement> = ({ currentTarget }) => {
		const inputValue = getInputValue(currentTarget);
		const inputName = currentTarget.name;
		onSettingsChange({ ...values, [inputName]: inputValue });
	};
</script>

<label class="label cursor-pointer">
	<span class="label-text">Show pomodoro</span>
	<input
		bind:checked={values.showPomodoro}
		on:change={handleChange}
		name="showPomodoro"
		id="showPomodoro"
		type="checkbox"
		class="toggle toggle-primary"
	/>
</label>

<div class="label flex justify-between items-center">
	<div class="label-text">Interval (in minutes)</div>
	<div class="flex gap-3 items-center">
		<button
			on:click={handleDecrease}
			class="btn btn-square btn-sm btn-primary"
			disabled={$interval === MIN_INTERVAL}>-5</button
		>
		{#key $interval}
			<div
				class="text-xl font-semibold text-primary min-w-10 text-center"
				in:scale={{ duration: 500, start: 1.5, easing: quintOut }}
			>
				{$interval}
			</div>
		{/key}
		<button on:click={handleIncrease} class="btn btn-square btn-sm btn-primary">+5</button>
	</div>
</div>

<label class="form-control w-full">
	<div class="label">
		<span class="label-text">Focus <strong>{values.focusTime} min</strong></span>
		<span class="label-text">Break <strong>{breakTime} min</strong></span>
	</div>
	<input
		bind:value={values.focusTime}
		on:input={handleChange}
		name="focusTime"
		id="clockSize"
		type="range"
		min="0"
		max={values.interval}
		step="1"
		class="range"
	/>
</label>
