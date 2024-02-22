<script lang="ts">
	import { WebviewWindow } from '@tauri-apps/api/window';
	import { Settings2 } from 'lucide-svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	let settingsWindow: WebviewWindow | null = null;

	const handleShowSetings = async () => {
		await invoke('open_settings_window');
	};
</script>

<button on:click={handleShowSetings} class="btn btn-square btn-primary absolute right-1 top-1 z-10">
	<Settings2 size="24" />
</button>
