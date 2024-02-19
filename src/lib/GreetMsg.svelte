<script>
	import { invoke } from '@tauri-apps/api/tauri';

	let name = '';
	let msg = '';

	async function greet() {
		msg = await invoke('greet', { name });
	}
</script>

<div>
	<input id="greet-input" placeholder="Enter a name..." bind:value={name} />
	<button on:click={greet}>Greet</button>
	<p>{msg}</p>
</div>
