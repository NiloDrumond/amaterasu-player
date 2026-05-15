<script lang="ts">
	import { page } from '$app/state';
	import { getPlayer } from '$lib/player/player.svelte';
	import AudioVisualizer from '$lib/components/player/audio-visualizer.svelte';

	type AlbumCoverCellProps = {
		id: string;
		title: string;
		coverUrl: string | null;
	};
	let { id, title, coverUrl }: AlbumCoverCellProps = $props();

	const player = getPlayer();
	const base = $derived(page.url.pathname.startsWith('/admin') ? '/admin/albums' : '/albums');
	const isPlaying = $derived(player.currentOrigin.albumId === id);
</script>

<a href={`${base}/${id}`} class="flex flex-row items-center gap-2">
	<span class="relative inline-block size-8 shrink-0">
		<img class="size-8" src={coverUrl ?? undefined} alt={title} />
		{#if isPlaying}
			<span class="absolute inset-0 flex items-center justify-center bg-black/65">
				<AudioVisualizer class="h-4 w-5" />
			</span>
		{/if}
	</span>
	<span class="overflow-hidden text-ellipsis">
		{title}
	</span>
</a>
