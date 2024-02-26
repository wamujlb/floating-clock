<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import type { ChangeEventHandler } from 'svelte/elements';
	import { getInputValue } from '$lib/utils';
	import { appStore } from '$lib/settings/store';
	import { serializeSettings, type SettingsPayload } from '$lib/settings/utils';
	import PomodoroSettings from '$lib/Pomodoro/PomodoroSettings.svelte';

	export let data: { data: App.Settings };

	let settings: App.Settings = data.data;

	const handleSettingsChange = async (newSettings: App.Settings) => {
		settings = newSettings;
		await invoke<SettingsPayload>('set_settings', {
			newSettings: JSON.stringify(serializeSettings(newSettings)),
		});
		await appStore.save();
	};

	const handleChange: ChangeEventHandler<HTMLInputElement> = ({ currentTarget }) => {
		handleSettingsChange({ ...settings, [currentTarget.name]: getInputValue(currentTarget) });
	};

	const handlePomodoroChange = (newValues: App.PomodoroSettings) => {
		handleSettingsChange({ ...settings, pomodoro: newValues });
	};
</script>

{#if settings}
	<main class="flex flex-col gap-2 pt-4 pb-10 px-6">
		<h1 class="text-2xl">Settings</h1>
		<div class="divider divider-start">Clock</div>

		<label class="label cursor-pointer">
			<span class="label-text">Show seconds</span>
			<input
				bind:checked={settings.showSeconds}
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
				<span class="label-text-alt">{`${(settings.opacity * 100).toFixed(0)}%`}</span>
			</div>
			<input
				bind:value={settings.opacity}
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
				<span class="label-text-alt">{settings.clockSize}</span>
			</div>
			<input
				bind:value={settings.clockSize}
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

		<PomodoroSettings values={{ ...settings.pomodoro }} onSettingsChange={handlePomodoroChange} />
	</main>
{/if}
