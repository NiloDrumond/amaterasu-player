<script lang="ts">
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import { toast } from 'svelte-sonner';
	import { getPlayer } from '$lib/player/player.svelte';
	import { getAlbumTracks } from '$lib/services/album-service';
	import { page } from '$app/state';

	const isAdmin = $derived(page.data.user?.role === 'admin');

	let {
		id,
		onEditTags,
	}: {
		id: string;
		onEditTags: () => void;
	} = $props();

	const player = getPlayer();

	async function play() {
		const { data } = await getAlbumTracks(fetch, id);
		if (!data) {
			toast.error('Failed to load tracks');
			return;
		}
		player.playQueue(data, 0, { albumId: id });
	}

	async function playNext() {
		const { data } = await getAlbumTracks(fetch, id);
		if (!data) {
			toast.error('Failed to load tracks');
			return;
		}
		player.playNext(data, { albumId: id });
	}

	async function playLater() {
		const { data } = await getAlbumTracks(fetch, id);
		if (!data) {
			toast.error('Failed to load tracks');
			return;
		}
		player.playLater(data, { albumId: id });
	}
</script>

<DropdownMenu.Group>
	<DropdownMenu.Label>Actions</DropdownMenu.Label>
	<DropdownMenu.Item onclick={play}>Play</DropdownMenu.Item>
	<DropdownMenu.Item onclick={playNext}>Play Next</DropdownMenu.Item>
	<DropdownMenu.Item onclick={playLater}>Play Later</DropdownMenu.Item>
	<DropdownMenu.Item onclick={onEditTags}>Edit Tags</DropdownMenu.Item>
	<DropdownMenu.Item onclick={() => navigator.clipboard.writeText(id)}>Copy ID</DropdownMenu.Item>
	{#if isAdmin}
		<DropdownMenu.Separator />
		<DropdownMenu.Item onclick={() => window.open(`/admin/albums/${id}`, '_blank', 'noopener')}>
			Edit (admin)
		</DropdownMenu.Item>
	{/if}
</DropdownMenu.Group>
