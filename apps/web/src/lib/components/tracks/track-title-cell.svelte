<script lang="ts">
	import type { TrackResponse } from '$lib/bindings/response/track/track-response';
	import { getPlayer } from '$lib/player/player.svelte';
	import AudioVisualizer from '$lib/components/player/audio-visualizer.svelte';

	let { track }: { track: TrackResponse } = $props();

	const player = getPlayer();
	const isPlaying = $derived(player.currentTrack?.id === track.id);
</script>

<span class="flex items-center gap-2">
	{#if isPlaying}
		<AudioVisualizer />
	{/if}
	<span class="overflow-hidden text-ellipsis whitespace-nowrap">{track.title}</span>
</span>
