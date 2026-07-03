<script lang="ts">
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import { toast } from 'svelte-sonner';
	import { deletePlaylist, getPlaylistTracks } from '$lib/services/playlist-service';
	import { invalidatePlaylistsCache } from '$lib/state/playlists-cache.svelte';
	import { pinPlaylist } from '$lib/services/pin-service';
	import { getPlayer } from '$lib/player/player.svelte';
	import type { PlaylistTrackResponse } from '$lib/bindings/response/playlist/playlist-track-response';
	import type { TrackResponse } from '$lib/bindings/response/track/track-response';

	let { id, onDeleted }: { id: string; onDeleted?: () => void } = $props();

	const player = getPlayer();

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

	async function play() {
		const { data } = await getPlaylistTracks(fetch, id);
		if (!data) {
			toast.error('Failed to load tracks');
			return;
		}
		player.playQueue(data.map(asTrackResponse), 0, { playlistId: id });
	}

	async function handleDelete() {
		const { error } = await deletePlaylist(fetch, id);
		if (error) {
			toast.error(error);
		} else {
			invalidatePlaylistsCache();
			toast.success('Playlist deleted');
			onDeleted?.();
		}
	}

	async function handlePin() {
		const { error } = await pinPlaylist(fetch, id);
		if (error) {
			toast.error(error);
		} else {
			toast.success('Pinned to home');
		}
	}
</script>

<DropdownMenu.Group>
	<DropdownMenu.Label>Actions</DropdownMenu.Label>
	<DropdownMenu.Item onclick={play}>Play</DropdownMenu.Item>
	<DropdownMenu.Item onclick={handlePin}>Pin to home</DropdownMenu.Item>
	<DropdownMenu.Item onclick={handleDelete} class="text-destructive focus:text-destructive">
		Delete
	</DropdownMenu.Item>
</DropdownMenu.Group>
