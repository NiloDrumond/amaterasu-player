<script lang="ts">
	import EllipsisIcon from '@lucide/svelte/icons/ellipsis';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import { toast } from 'svelte-sonner';
	import { removeTrackFromPlaylist } from '$lib/services/playlist-service';

	let {
		playlistId,
		playlistTrackId,
		onRemoved,
	}: { playlistId: string; playlistTrackId: string; onRemoved?: () => void } = $props();

	async function removeTrack() {
		const { error } = await removeTrackFromPlaylist(fetch, playlistId, playlistTrackId);
		if (error) {
			toast.error(error);
		} else {
			toast.success('Track removed');
			onRemoved?.();
		}
	}
</script>

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
			<DropdownMenu.Item onclick={removeTrack} class="text-destructive focus:text-destructive">
				Remove from playlist
			</DropdownMenu.Item>
		</DropdownMenu.Group>
	</DropdownMenu.Content>
</DropdownMenu.Root>
