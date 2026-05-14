<script lang="ts">
	import { Icons } from '$lib/components/ui/icons';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import AddToPlaylistDialog from '$lib/components/playlists/add-to-playlist-dialog.svelte';
	import TagPickerDialog from '$lib/components/tags/tag-picker-dialog.svelte';
	import type { TrackResponse } from '$lib/bindings/response/track/track-response';
	import TrackActionItems from './track-action-items.svelte';

	let { track }: { track: TrackResponse } = $props();

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
				<Icons.More />
			</Button>
		{/snippet}
	</DropdownMenu.Trigger>
	<DropdownMenu.Content>
		<TrackActionItems
			{track}
			onAddToPlaylist={() => (addToPlaylistOpen = true)}
			onEditTags={() => (tagsOpen = true)}
		/>
	</DropdownMenu.Content>
</DropdownMenu.Root>
