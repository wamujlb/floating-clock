<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import type { ChangeEventHandler } from 'svelte/elements';
	import { initSettings } from '$lib/utils';

	let settings = initSettings;

	const handleSettingsChange = async (newSettings: App.Settings) => {
		settings = newSettings;
		const res = await invoke<App.SettingsChangePayload>('set_settings', {
			newSettings: JSON.stringify({
				show_seconds: settings.showSeconds,
				opacity: settings.opacity,
				clock_size: settings.clockSize,
				variant: settings.variant,
			}),
		});
	};

	const handleChange: ChangeEventHandler<HTMLInputElement> = ({ currentTarget }) => {
		let value;
		switch (currentTarget.type) {
			case 'checkbox':
				value = currentTarget.checked;
				break;
			case 'number':
			case 'range':
				value = currentTarget.valueAsNumber;
				break;
			default:
				value = currentTarget.value;
				break;
		}
		handleSettingsChange({ ...settings, [currentTarget.name]: value });
	};
</script>

<main class="flex flex-col gap-4 mt-4 px-6">
	<h1 class="text-2xl">Settings</h1>
	<hr class="border-slate-600" />
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
			<span class="label-text-alt">{`${settings.clockSize}px`}</span>
		</div>
		<input
			bind:value={settings.clockSize}
			on:input={handleChange}
			name="clockSize"
			id="clockSize"
			type="range"
			min="30"
			max="500"
			step="10"
			class="range"
		/>
	</label>

	<select class="select select-bordered w-full mt-4" value={settings.variant}>
		<option disabled selected>Clock style</option>
		<option value="Flip">Flip clock</option>
		<option value="Digital">Digital</option>
	</select>
</main>
