<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { reorderPlaylistTrack, updatePlaylistFilter } from '$lib/services/playlist-service';
	import { getPlayer } from '$lib/player/player.svelte';
	import { Button } from '$lib/components/ui/button';
	import { formatMilliseconds } from '$lib/utils/date';
	import { shuffle } from '$lib/utils/shuffle';
	import { toast } from 'svelte-sonner';
	import { draggable, droppable, type DragDropState } from '@thisux/sveltednd';
	import type { PlaylistTrackResponse } from '$lib/bindings/response/playlist/playlist-track-response';
	import type { TrackResponse } from '$lib/bindings/response/track/track-response';
	import type { FilterNode } from '$lib/bindings/filter/filter-node';
	import PlaylistTrackActions from '$lib/components/playlists/playlist-track-actions.svelte';
	import PlaylistTrackRowContextMenu from '$lib/components/playlists/playlist-track-row-context-menu.svelte';
	import TrackRowContextMenu from '$lib/components/tracks/track-row-context-menu.svelte';
	import { tracksColumns } from '$lib/components/tracks/columns.js';
	import DataTable from '$lib/components/ui/data-table/data-table.svelte';
	import FilterBar from '$lib/components/filters/filter-bar.svelte';
	import SearchInput from '$lib/components/filters/search-input.svelte';
	import { getTextSearch, setTextSearch } from '$lib/utils/filter-url';
	import PlayIcon from '@lucide/svelte/icons/play';
	import ShuffleIcon from '@lucide/svelte/icons/shuffle';
	import ListStartIcon from '@lucide/svelte/icons/list-start';
	import ListEndIcon from '@lucide/svelte/icons/list-end';
	import ListMusicIcon from '@lucide/svelte/icons/list-music';
	import SparklesIcon from '@lucide/svelte/icons/sparkles';
	import GripVerticalIcon from '@lucide/svelte/icons/grip-vertical';

	let { data } = $props();
	const player = getPlayer();

	const isDynamic = $derived(data.playlist.playlistType === 'dynamic');
	// Writable derived: tracks `data.playlist.filterDefinition` by default, but
	// `onDynamicFilterChange` assigns to it for optimistic local updates.
	let dynamicFilter: FilterNode | null = $derived(data.playlist.filterDefinition ?? null);

	async function onDynamicFilterChange(next: FilterNode | null) {
		dynamicFilter = next;
		if (!next) return;
		const { error } = await updatePlaylistFilter(fetch, data.playlist.id, {
			filterDefinition: next,
		});
		if (error) {
			toast.error('Failed to update filter');
			return;
		}
		await invalidateAll();
	}

	// Local override for optimistic drag reorder; null means "use server data"
	let localTracks = $state<PlaylistTrackResponse[] | null>(null);

	const tracks = $derived(localTracks ?? data.tracks);

	function asTrackResponse(t: PlaylistTrackResponse): TrackResponse {
		const {
			playlistTrackId: _playlistTrackId,
			position: _position,
			addedAt: _addedAt,
			artist,
			album,
			...rest
		} = t;
		return {
			...rest,
			artist: artist ? { id: artist.id, name: artist.name } : null,
			album: album ? { id: album.id, title: album.title, coverUrl: album.coverUrl ?? null } : null,
		} as TrackResponse;
	}

	function play() {
		player.playQueue(tracks.map(asTrackResponse), 0, { playlistId: data.playlist.id });
	}

	function shufflePlay() {
		player.playQueue(shuffle(tracks.map(asTrackResponse)), 0, {
			playlistId: data.playlist.id,
		});
	}

	function playNext() {
		player.playNext(tracks.map(asTrackResponse));
	}

	function playLater() {
		player.playLater(tracks.map(asTrackResponse));
	}

	// For dynamic playlists, the server returns 0 for these aggregates (they're
	// derived from playlist_tracks which dynamic playlists don't use). Compute
	// from the resolved track list instead.
	const trackCount = $derived(isDynamic ? tracks.length : Number(data.playlist.trackCount));
	const totalDurationMs = $derived(
		isDynamic
			? tracks.reduce((sum, t) => sum + t.durationMs, 0)
			: Number(data.playlist.totalDurationMs),
	);
	const duration = $derived(formatMilliseconds(totalDurationMs));

	async function handleDrop(state: DragDropState<PlaylistTrackResponse>) {
		const { draggedItem, targetContainer, dropPosition } = state;
		if (!draggedItem || targetContainer == null) return;

		const draggedPlaylistTrackId = draggedItem.playlistTrackId;
		const targetId = targetContainer;

		if (draggedPlaylistTrackId === targetId) return;

		const current = [...tracks];
		const fromIndex = current.findIndex((t) => t.playlistTrackId === draggedPlaylistTrackId);
		let toIndex = current.findIndex((t) => t.playlistTrackId === targetId);
		if (fromIndex === -1 || toIndex === -1) return;

		if (dropPosition === 'after') toIndex++;

		// Optimistic update
		const updated = [...current];
		const [moved] = updated.splice(fromIndex, 1);
		const adjusted = fromIndex < toIndex ? toIndex - 1 : toIndex;
		updated.splice(adjusted, 0, moved);
		localTracks = updated;

		// Determine afterId: the track immediately before the dragged item in new order.
		// The server queries by track_id (not playlist_track_id), so use .id here.
		const newIndex = updated.findIndex((t) => t.playlistTrackId === draggedPlaylistTrackId);
		const afterTrack = newIndex > 0 ? updated[newIndex - 1] : null;
		const afterId = afterTrack?.id ?? null;

		const { error } = await reorderPlaylistTrack(fetch, data.playlist.id, draggedItem.id, {
			afterId,
		});

		if (error) {
			toast.error('Failed to reorder track');
			localTracks = null;
		} else {
			await invalidateAll();
			localTracks = null;
		}
	}
</script>

<div class="flex flex-col gap-6 p-6">
	<!-- Hero -->
	<div class="flex flex-row items-end gap-6">
		<div
			class="flex size-48 shrink-0 items-center justify-center rounded-lg bg-muted text-muted-foreground"
		>
			{#if isDynamic}
				<SparklesIcon class="size-16 opacity-30" />
			{:else}
				<ListMusicIcon class="size-16 opacity-30" />
			{/if}
		</div>
		<div class="flex min-w-0 flex-col gap-1">
			<p class="text-xs font-medium tracking-widest text-muted-foreground uppercase">
				{isDynamic ? 'Dynamic Playlist' : 'Playlist'}
			</p>
			<h1 class="truncate text-3xl">{data.playlist.name}</h1>
			<p class="text-sm text-muted-foreground">
				{[`${trackCount} tracks`, duration].filter(Boolean).join(' · ')}
			</p>
		</div>
	</div>

	{#if isDynamic}
		<div class="flex flex-col gap-2">
			<p class="text-xs tracking-widest text-muted-foreground uppercase">Filters</p>
			<div class="flex flex-row flex-wrap items-center gap-2">
				<SearchInput
					value={getTextSearch(dynamicFilter)}
					onChange={(q) => onDynamicFilterChange(setTextSearch(dynamicFilter, q))}
					placeholder="Search tracks…"
				/>
				<FilterBar entity="tracks" filter={dynamicFilter} onChange={onDynamicFilterChange} />
			</div>
		</div>
	{/if}

	<!-- Toolbar -->
	<div class="flex flex-row flex-wrap items-center gap-2">
		<Button onclick={play} class="gap-2">
			<PlayIcon class="size-4" />
			Play
		</Button>
		<Button variant="ghost" onclick={shufflePlay} class="gap-2">
			<ShuffleIcon class="size-4" />
			Shuffle
		</Button>
		<Button variant="ghost" onclick={playNext} class="gap-2">
			<ListStartIcon class="size-4" />
			Play Next
		</Button>
		<Button variant="ghost" onclick={playLater} class="gap-2">
			<ListEndIcon class="size-4" />
			Play Later
		</Button>
	</div>

	<!-- Tracks table -->
	{#if tracks.length === 0}
		<p class="text-sm text-muted-foreground">No tracks in this playlist yet.</p>
	{:else if isDynamic}
		{@const tracksAsResponse = tracks.map(asTrackResponse)}
		<DataTable
			data={tracksAsResponse}
			columns={tracksColumns.filter((col) => col.id !== 'trackNo')}
			onRowClick={(_row, index) =>
				player.playQueue(tracksAsResponse, index, { playlistId: data.playlist.id })}
		>
			{#snippet rowContextMenu({ row, trigger })}
				<TrackRowContextMenu track={row} {trigger} />
			{/snippet}
		</DataTable>
	{:else}
		<div class="flex flex-col">
			<!-- Table header -->
			<div
				class="grid grid-cols-[40px_1fr_180px_80px_50px] border-b px-2 pb-2 text-xs font-medium tracking-wider text-muted-foreground uppercase"
			>
				<span></span>
				<span>TITLE</span>
				<span>ARTIST</span>
				<span class="text-right">TIME</span>
				<span></span>
			</div>

			{#each tracks as track (track.playlistTrackId)}
				<PlaylistTrackRowContextMenu
					playlistId={data.playlist.id}
					playlistTrackId={track.playlistTrackId}
					onRemoved={() => invalidateAll()}
				>
					{#snippet trigger({ props })}
						<div
							{...props}
							class="group grid grid-cols-[40px_1fr_180px_80px_50px] items-center border-b px-2 py-1 transition-colors hover:bg-muted/50"
							use:draggable={{
								container: track.playlistTrackId,
								dragData: track,
								handle: '.drag-handle',
							}}
							use:droppable={{
								container: track.playlistTrackId,
								callbacks: { onDrop: handleDrop },
							}}
						>
							<!-- Drag handle -->
							<div
								class="drag-handle flex cursor-grab items-center justify-center text-muted-foreground opacity-0 transition-opacity group-hover:opacity-100 active:cursor-grabbing"
							>
								<GripVerticalIcon class="size-4" />
							</div>

							<!-- Title -->
							<button
								class="min-w-0 truncate text-left text-sm font-semibold"
								onclick={() => {
									const i = tracks.findIndex((t) => t.playlistTrackId === track.playlistTrackId);
									player.playQueue(tracks.map(asTrackResponse), i, {
										playlistId: data.playlist.id,
									});
								}}
							>
								{track.title}
							</button>

							<!-- Artist -->
							<span class="min-w-0 truncate text-sm text-muted-foreground">
								{#if track.artist}
									<a
										href={`/artists/${track.artist.id}`}
										class="transition-colors hover:text-foreground"
										onclick={(e) => e.stopPropagation()}
									>
										{track.artist.name}
									</a>
								{/if}
							</span>

							<!-- Duration -->
							<span class="text-right text-sm text-muted-foreground">
								{formatMilliseconds(track.durationMs)}
							</span>

							<!-- Actions -->
							<div class="flex justify-end">
								<PlaylistTrackActions
									playlistId={data.playlist.id}
									playlistTrackId={track.playlistTrackId}
									onRemoved={() => invalidateAll()}
								/>
							</div>
						</div>
					{/snippet}
				</PlaylistTrackRowContextMenu>
			{/each}
		</div>
	{/if}
</div>
