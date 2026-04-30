<script lang="ts">
	import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
	import AddToPlaylistDialog from '$lib/components/playlists/add-to-playlist-dialog.svelte';
	import TagPickerDialog from '$lib/components/tags/tag-picker-dialog.svelte';
	import type { TrackResponse } from '$lib/bindings/response/track/track-response';
	import type { Snippet } from 'svelte';
	import TrackActionItems from './track-action-items.svelte';

	let {
		track,
		trigger,
	}: {
		track: TrackResponse;
		trigger: Snippet<[{ props: Record<string, unknown> }]>;
	} = $props();

	let addToPlaylistOpen = $state(false);
	let tagsOpen = $state(false);
</script>

<AddToPlaylistDialog
	tracks={[track]}
	bind:open={addToPlaylistOpen}
	onClose={() => (addToPlaylistOpen = false)}
/>

<TagPickerDialog entity="track" entityId={track.id} bind:open={tagsOpen} />

<ContextMenu.Root>
	<ContextMenu.Trigger>
		{#snippet child({ props })}
			{@render trigger({ props })}
		{/snippet}
	</ContextMenu.Trigger>
	<ContextMenu.Content>
		<TrackActionItems
			{track}
			onAddToPlaylist={() => (addToPlaylistOpen = true)}
			onEditTags={() => (tagsOpen = true)}
		/>
	</ContextMenu.Content>
</ContextMenu.Root>
