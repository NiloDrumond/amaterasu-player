<script lang="ts">
	import type { PlaylistResponse } from '$lib/bindings/response/playlist/playlist-response';
	import { getPlayer } from '$lib/player/player.svelte';
	import AudioVisualizer from '$lib/components/player/audio-visualizer.svelte';

	let { playlist }: { playlist: PlaylistResponse } = $props();

	const player = getPlayer();
	const isPlaying = $derived(player.currentOrigin.playlistId === playlist.id);
</script>

<span class="flex items-center gap-2">
	{#if isPlaying}
		<AudioVisualizer />
	{/if}
	<span class="overflow-hidden text-ellipsis whitespace-nowrap">{playlist.name}</span>
</span>
