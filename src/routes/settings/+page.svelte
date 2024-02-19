<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import type { ChangeEventHandler } from 'svelte/elements';

	let settings: App.Settings = {
		showSeconds: true,
		opacity: 1,
		clockSize: 400,
	};

	const handleSettingsChange = async (newSettings: App.Settings) => {
		settings = newSettings;
		console.log(newSettings);
		const res = await invoke<App.SettingsChangePayload>('set_settings', {
			newSettings: JSON.stringify({
				show_seconds: settings.showSeconds,
				opacity: settings.opacity,
				clock_size: settings.clockSize,
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

<main>
	<label for="showSeconds">
		<input
			bind:checked={settings.showSeconds}
			on:change={handleChange}
			name="showSeconds"
			id="showSeconds"
			type="checkbox"
		/>
		Show seconds
	</label>

	<label for="opacity">
		<input
			bind:value={settings.opacity}
			on:input={handleChange}
			name="opacity"
			id="opacity"
			type="range"
			min="0.1"
			max="1"
			step="0.01"
		/>
		Opacity
	</label>

	<label for="clockSize">
		<input
			bind:value={settings.clockSize}
			on:input={handleChange}
			name="clockSize"
			id="clockSize"
			type="range"
			min="200"
			max="1000"
			step="10"
		/>
		Clock size
	</label>
</main>

<style>
	main {
		display: flex;
		flex-direction: column;
		gap: 8px;
	}
</style>
