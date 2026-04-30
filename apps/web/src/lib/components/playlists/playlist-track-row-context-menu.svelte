<script lang="ts">
	import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
	import type { Snippet } from 'svelte';
	import PlaylistTrackActionItems from './playlist-track-action-items.svelte';

	let {
		playlistId,
		playlistTrackId,
		onRemoved,
		trigger,
	}: {
		playlistId: string;
		playlistTrackId: string;
		onRemoved?: () => void;
		trigger: Snippet<[{ props: Record<string, unknown> }]>;
	} = $props();
</script>

<ContextMenu.Root>
	<ContextMenu.Trigger>
		{#snippet child({ props })}
			{@render trigger({ props })}
		{/snippet}
	</ContextMenu.Trigger>
	<ContextMenu.Content>
		<PlaylistTrackActionItems {playlistId} {playlistTrackId} {onRemoved} />
	</ContextMenu.Content>
</ContextMenu.Root>
