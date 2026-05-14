<script lang="ts">
	import { Icons } from '$lib/components/ui/icons';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import PlaylistTrackActionItems from './playlist-track-action-items.svelte';

	let {
		playlistId,
		playlistTrackId,
		onRemoved,
	}: { playlistId: string; playlistTrackId: string; onRemoved?: () => void } = $props();
</script>

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
		<PlaylistTrackActionItems {playlistId} {playlistTrackId} {onRemoved} />
	</DropdownMenu.Content>
</DropdownMenu.Root>
