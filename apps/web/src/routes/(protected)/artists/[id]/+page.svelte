<script lang="ts">
	import { albumsColumns } from '$lib/components/albums/columns.js';
	import { tracksColumns } from '$lib/components/tracks/columns';
	import DataTable from '$lib/components/ui/data-table/data-table.svelte';
	import { Button } from '$lib/components/ui/button';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { getPlayer } from '$lib/player/player.svelte';
	import MicVocalIcon from '@lucide/svelte/icons/mic-vocal';
	import PencilIcon from '@lucide/svelte/icons/pencil';

	let { data } = $props();
	const player = getPlayer();
	const isAdmin = $derived(page.data.user?.role === 'admin');

	const artistAlbumColumns = albumsColumns.filter((col) => col.id !== 'artist');
	const artistTrackColumns = tracksColumns.filter(
		(col) => !('accessorKey' in col && col.accessorKey === 'artist'),
	);
</script>

<div class="flex flex-col gap-6 p-6">
	<div class="flex flex-row items-end gap-6">
		<div
			class="flex size-48 shrink-0 items-center justify-center rounded-lg bg-muted text-muted-foreground"
		>
			<MicVocalIcon class="size-16 opacity-30" />
		</div>
		<div class="flex min-w-0 flex-col gap-1">
			<p class="text-xs font-medium tracking-widest text-muted-foreground uppercase">Artist</p>
			<h1 class="truncate text-3xl">{data.artist.name}</h1>
			<p class="text-sm text-muted-foreground">
				{String(data.artist.albumCount)}
				{Number(data.artist.albumCount) === 1 ? 'album' : 'albums'}
			</p>
		</div>
	</div>

	{#if isAdmin}
		<div class="flex flex-row flex-wrap items-center gap-2">
			<Button
				variant="ghost"
				onclick={() => window.open(`/admin/artists/${data.artist.id}`, '_blank', 'noopener')}
				class="gap-2"
			>
				<PencilIcon class="size-4" />
				Edit (admin)
			</Button>
		</div>
	{/if}

	<DataTable
		storageKey="artist:albums"
		data={data.albums}
		columns={artistAlbumColumns}
		onRowClick={(row) => goto(`/albums/${row.id}`)}
	/>

	<DataTable
		storageKey="artist:tracks"
		data={data.tracks}
		columns={artistTrackColumns}
		onRowClick={(_, index) => player.playQueue(data.tracks, index)}
	/>
</div>
