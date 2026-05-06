<script lang="ts">
	import { albumsColumns } from '$lib/components/albums/columns.js';
	import DataTable from '$lib/components/ui/data-table/data-table.svelte';
	import { goto } from '$app/navigation';
	import MicVocalIcon from '@lucide/svelte/icons/mic-vocal';

	let { data } = $props();

	const artistColumns = albumsColumns.filter((col) => col.id !== 'artist');
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

	<DataTable
		data={data.albums}
		columns={artistColumns}
		onRowClick={(row) => goto(`/albums/${row.id}`)}
	/>
</div>
