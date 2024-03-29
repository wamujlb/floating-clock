<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { writable } from 'svelte/store';
	import type { ChangeEventHandler } from 'svelte/elements';
	import { getInputValue } from '$lib/utils';
	import { appStore } from '$lib/settings/store';
	import { serializeSettings, type SettingsPayload } from '$lib/settings/utils';
	import PomodoroSettings from '$lib/pomodoro/PomodoroSettings.svelte';
	import { onDestroy } from 'svelte';

	export let data: { data: App.Settings };

	let settings = writable<App.Settings>(data.data);

	const unsubscribe = settings.subscribe(async (settings) => {
		const serializedSettings = JSON.stringify(serializeSettings(settings));
		await invoke<SettingsPayload>('set_settings', { newSettings: serializedSettings });
		await appStore.set('settings', serializedSettings);
		await appStore.save();
	});

	const handleChange: ChangeEventHandler<HTMLInputElement> = ({ currentTarget }) => {
		settings.update((settings) => ({
			...settings,
			[currentTarget.name]: getInputValue(currentTarget),
		}));
	};

	const handleVariantSelect: ChangeEventHandler<HTMLSelectElement> = ({ currentTarget }) => {
		settings.update((settings) => ({
			...settings,
			[currentTarget.name]: currentTarget.value as App.ClockVariant,
		}));
	};

	const handlePomodoroChange = (newValues: App.PomodoroSettings) => {
		settings.update((settings) => ({ ...settings, pomodoro: newValues }));
	};

	onDestroy(() => {
		unsubscribe();
	});
</script>

{#if $settings}
	<main class="flex flex-col gap-2 pt-4 pb-10 px-6">
		<h1 class="text-2xl">Settings</h1>
		<div class="divider divider-start">Clock</div>

		<label class="form-control w-full">
			<div class="label">
				<span class="label-text">Clock type</span>
			</div>
			<select
				name="variant"
				bind:value={$settings.variant}
				on:select={handleVariantSelect}
				class="select select-bordered"
			>
				<option value="Flip">Flip clock</option>
				<option value="BinaryAnalog">Binary analog clock</option>
			</select>
		</label>

		<label class="label cursor-pointer">
			<span class="label-text">Show seconds</span>
			<input
				bind:checked={$settings.showSeconds}
				on:change={handleChange}
				name="showSeconds"
				id="showSeconds"
				type="checkbox"
				class="toggle toggle-primary"
			/>
		</label>

		<label class="form-control w-full">
			<div class="label">
				<span class="label-text">Opacity</span>
				<span class="label-text-alt">{`${($settings.opacity * 100).toFixed(0)}%`}</span>
			</div>
			<input
				bind:value={$settings.opacity}
				on:input={handleChange}
				name="opacity"
				id="opacity"
				type="range"
				min="0.1"
				max="1"
				step="0.01"
				class="range"
			/>
		</label>

		<label class="form-control w-full">
			<div class="label">
				<span class="label-text">Clock size</span>
				<span class="label-text-alt">{$settings.clockSize}</span>
			</div>
			<input
				bind:value={$settings.clockSize}
				on:input={handleChange}
				name="clockSize"
				id="clockSize"
				type="range"
				min="10"
				max="300"
				step="2"
				class="range"
			/>
		</label>

		<div class="divider divider-start">Pomodoro</div>

		<PomodoroSettings values={{ ...$settings.pomodoro }} onSettingsChange={handlePomodoroChange} />
	</main>
{/if}
