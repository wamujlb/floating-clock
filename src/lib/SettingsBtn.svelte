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

<button on:click={handleTrigger} class="btn btn-square btn-primary absolute right-1 top-1 z-10">
	<Settings2 size="24" />
</button>
