<script lang="ts">
	import EllipsisIcon from '@lucide/svelte/icons/ellipsis';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import AddToPlaylistDialog from '$lib/components/playlists/add-to-playlist-dialog.svelte';
	import TagPickerDialog from '$lib/components/tags/tag-picker-dialog.svelte';
	import type { TrackResponse } from '$lib/bindings/response/track/track-response';
	import { getPlayer } from '$lib/player/player.svelte';

	let { id, track }: { id: string; track: TrackResponse } = $props();

	const player = getPlayer();
	let addToPlaylistOpen = $state(false);
	let tagsOpen = $state(false);
</script>

<AddToPlaylistDialog
	tracks={[track]}
	bind:open={addToPlaylistOpen}
	onClose={() => (addToPlaylistOpen = false)}
/>

<TagPickerDialog entity="track" entityId={track.id} bind:open={tagsOpen} />

<DropdownMenu.Root>
	<DropdownMenu.Trigger>
		{#snippet child({ props })}
			<Button {...props} variant="ghost" size="icon" class="relative size-8 p-0">
				<span class="sr-only">Open menu</span>
				<EllipsisIcon />
			</Button>
		{/snippet}
	</DropdownMenu.Trigger>
	<DropdownMenu.Content>
		<DropdownMenu.Group>
			<DropdownMenu.Label>Actions</DropdownMenu.Label>
			<DropdownMenu.Item onclick={() => player.playNext([track])}>Play Next</DropdownMenu.Item>
			<DropdownMenu.Item onclick={() => player.playLater([track])}>Play Later</DropdownMenu.Item>
			<DropdownMenu.Item onclick={() => (addToPlaylistOpen = true)}>
				Add to Playlist
			</DropdownMenu.Item>
			<DropdownMenu.Item onclick={() => (tagsOpen = true)}>Edit Tags</DropdownMenu.Item>
		</DropdownMenu.Group>
	</DropdownMenu.Content>
</DropdownMenu.Root>
