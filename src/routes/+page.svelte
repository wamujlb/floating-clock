<script lang="ts">
	import { fade } from 'svelte/transition';
	import Clock from '$lib/Clock.svelte';
	import SettingsBtn from '$lib/SettingsBtn.svelte';
	import { writable } from 'svelte/store';

	let isHovered = writable(false);
</script>

<main
	class="relative"
	on:mouseenter={() => isHovered.set(true)}
	on:mouseleave={() => isHovered.set(false)}
>
	{#if $isHovered}
		<div transition:fade={{ delay: 100, duration: 300 }}>
			<SettingsBtn />
		</div>
	{/if}

	<Clock />
</main>

<style>
	:global(html) {
		background-color: transparent !important;
	}
</style>
