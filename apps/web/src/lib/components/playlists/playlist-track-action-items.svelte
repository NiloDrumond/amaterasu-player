<script lang="ts">
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

<DropdownMenu.Group>
	<DropdownMenu.Label>Actions</DropdownMenu.Label>
	<DropdownMenu.Item onclick={removeTrack} class="text-destructive focus:text-destructive">
		Remove from playlist
	</DropdownMenu.Item>
</DropdownMenu.Group>
