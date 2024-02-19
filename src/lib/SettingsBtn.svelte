<script lang="ts">
	import { WebviewWindow } from '@tauri-apps/api/window';
	import { Settings2 } from 'lucide-svelte';

	let settingsWindow: WebviewWindow | null = null;

	const handleShowSetings = () => {
		settingsWindow = new WebviewWindow('settings', {
			url: '/settings',
			width: 400,
			height: 400,
			resizable: false,
			center: true,
			closable: true,
			minimizable: false,
			focus: true,
		});

		settingsWindow.onCloseRequested(() => {
			settingsWindow = null;
			console.log('settings on close requested');
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

<button on:click={handleTrigger}>
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
