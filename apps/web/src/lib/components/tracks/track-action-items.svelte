<script lang="ts">
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import type { TrackResponse } from '$lib/bindings/response/track/track-response';
	import { getPlayer } from '$lib/player/player.svelte';
	import { page } from '$app/state';

	let {
		track,
		onAddToPlaylist,
		onEditTags,
	}: {
		track: TrackResponse;
		onAddToPlaylist: () => void;
		onEditTags: () => void;
	} = $props();

	const player = getPlayer();
	const isAdmin = $derived(page.data.user?.role === 'admin');
</script>

<DropdownMenu.Group>
	<DropdownMenu.Label>Actions</DropdownMenu.Label>
	<DropdownMenu.Item onclick={() => player.playNext([track])}>Play Next</DropdownMenu.Item>
	<DropdownMenu.Item onclick={() => player.playLater([track])}>Play Later</DropdownMenu.Item>
	<DropdownMenu.Item onclick={onAddToPlaylist}>Add to Playlist</DropdownMenu.Item>
	<DropdownMenu.Item onclick={onEditTags}>Edit Tags</DropdownMenu.Item>
	{#if isAdmin}
		<DropdownMenu.Separator />
		<DropdownMenu.Item
			onclick={() => window.open(`/admin/tracks/${track.id}`, '_blank', 'noopener')}
		>
			Edit (admin)
		</DropdownMenu.Item>
	{/if}
</DropdownMenu.Group>
