<script lang="ts">
	import { albumsColumns } from '$lib/components/albums/columns.js';
	import DataTable from '$lib/components/ui/data-table/data-table.svelte';
	import AlbumRowContextMenu from '$lib/components/albums/album-row-context-menu.svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';

	let { data } = $props();

	function onChangePage(newPage: number) {
		if (!data.albums) return;
		let url = new URL(page.url);
		url.searchParams.set('offset', (data.albums.limit * (newPage - 1)).toString());
		goto(url);
	}
</script>

{#if data.error}
	<h1>Error</h1>
	<p>{data.error.message}</p>
{:else}
	<div class="flex flex-col p-4">
		<h1>ALBUMS</h1>
		<DataTable
			data={data.albums.data}
			columns={albumsColumns}
			pagination={{
				page: data.page,
				totalPages: Math.ceil(Number(data.albums.total) / data.albums.limit),
				onChangePage: onChangePage,
			}}
			onRowClick={(row) => goto(`/albums/${row.id}`)}
		>
			{#snippet rowContextMenu({ row, trigger })}
				<AlbumRowContextMenu id={row.id} {trigger} />
			{/snippet}
		</DataTable>
	</div>
{/if}
