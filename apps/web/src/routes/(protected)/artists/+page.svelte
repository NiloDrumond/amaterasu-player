<script lang="ts">
	import { artistsColumns } from '$lib/components/artists/columns.js';
	import DataTable from '$lib/components/ui/data-table/data-table.svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';

	let { data } = $props();

	function onChangePage(newPage: number) {
		if (!data.artists) return;
		let url = new URL(page.url);
		url.searchParams.set('offset', (data.artists.limit * (newPage - 1)).toString());
		goto(url);
	}
</script>

{#if data.error}
	<h1>Error</h1>
	<p>{data.error.message}</p>
{:else}
	<div class="flex flex-col p-4">
		<h1>ARTISTS</h1>
		<DataTable
			data={data.artists.data}
			columns={artistsColumns}
			pagination={{
				page: data.page,
				totalPages: Math.ceil(Number(data.artists.total) / data.artists.limit),
				onChangePage: onChangePage,
			}}
			onRowClick={(row) => goto(`/artists/${row.id}`)}
		/>
	</div>
{/if}
