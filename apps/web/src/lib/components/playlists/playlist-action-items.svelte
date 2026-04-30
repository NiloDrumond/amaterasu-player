<script lang="ts">
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import { toast } from 'svelte-sonner';
	import { deletePlaylist } from '$lib/services/playlist-service';

	let { id, onDeleted }: { id: string; onDeleted?: () => void } = $props();

	async function handleDelete() {
		const { error } = await deletePlaylist(fetch, id);
		if (error) {
			toast.error(error);
		} else {
			toast.success('Playlist deleted');
			onDeleted?.();
		}
	}
</script>

<DropdownMenu.Group>
	<DropdownMenu.Label>Actions</DropdownMenu.Label>
	<DropdownMenu.Item onclick={handleDelete} class="text-destructive focus:text-destructive">
		Delete
	</DropdownMenu.Item>
</DropdownMenu.Group>
