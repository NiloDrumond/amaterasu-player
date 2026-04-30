<script lang="ts">
	import EllipsisIcon from '@lucide/svelte/icons/ellipsis';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import { toast } from 'svelte-sonner';
	import { getPlayer } from '$lib/player/player.svelte';
	import { getAlbumTracks } from '$lib/services/album-service';
	import TagPickerDialog from '$lib/components/tags/tag-picker-dialog.svelte';

	let { id }: { id: string } = $props();

	const player = getPlayer();
	let tagsOpen = $state(false);

	async function playNext() {
		const { data } = await getAlbumTracks(fetch, id);
		if (!data) {
			toast.error('Failed to load tracks');
			return;
		}
		player.playNext(data);
	}

	async function playLater() {
		const { data } = await getAlbumTracks(fetch, id);
		if (!data) {
			toast.error('Failed to load tracks');
			return;
		}
		player.playLater(data);
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
			<DropdownMenu.Item onclick={playNext}>Play Next</DropdownMenu.Item>
			<DropdownMenu.Item onclick={playLater}>Play Later</DropdownMenu.Item>
			<DropdownMenu.Item onclick={() => (tagsOpen = true)}>Edit Tags</DropdownMenu.Item>
			<DropdownMenu.Item onclick={() => navigator.clipboard.writeText(id)}>
				Copy ID
			</DropdownMenu.Item>
		</DropdownMenu.Group>
	</DropdownMenu.Content>
</DropdownMenu.Root>

<TagPickerDialog entity="album" entityId={id} bind:open={tagsOpen} />
