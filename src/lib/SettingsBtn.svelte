<script lang="ts">
	import { WebviewWindow } from '@tauri-apps/api/window';
	import { Settings2 } from 'lucide-svelte';

	let settingsWindow: WebviewWindow | null = null;

	const handleShowSetings = () => {
		settingsWindow = new WebviewWindow('settings', {
			url: '/settings',
			title: 'Settings',
			width: 400,
			height: 600,
			resizable: false,
			center: true,
			closable: true,
			minimizable: false,
			focus: true,
			alwaysOnTop: true,
		});

		settingsWindow.onCloseRequested(() => {
			settingsWindow = null;
		});
	};

	const handleTrigger = () => {
		if (!settingsWindow) {
			handleShowSetings();
			return;
		}

		settingsWindow.close();
		settingsWindow = null;
	};
</script>

<button on:click={handleTrigger} class="btn btn-square">
	<Settings2 color="white" size="24" />
</button>

<style>
	button {
		position: absolute;
		top: 0;
		right: 0;
		background-color: blue;
		color: white;
		border: none;
		border-radius: 4px;
	}
</style>
