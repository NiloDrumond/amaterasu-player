<script lang="ts">
	import { tracksColumns } from '$lib/components/tracks/columns.js';
	import DataTable from '$lib/components/ui/data-table/data-table.svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { getPlayer } from '$lib/player/player.svelte';

	let { data } = $props();
	const player = getPlayer();

	function onChangePage(newPage: number) {
		if (!data.tracks) return;
		let url = new URL(page.url);
		url.searchParams.set('offset', (data.tracks.limit * (newPage - 1)).toString());
		goto(url);
	}
</script>

{#if data.error}
	<h1>Error</h1>
	<p>{data.error.message}</p>
{:else}
	<div class="flex flex-col p-4">
		<h1>TRACKS</h1>
		<div>TODO: Filters</div>
		<DataTable
			data={data.tracks.data}
			columns={tracksColumns.filter((col) => col.id !== 'trackNo')}
			pagination={{
				page: data.page,
				totalPages: Math.ceil(Number(data.tracks.total) / data.tracks.limit),
				onChangePage: onChangePage,
			}}
			onRowClick={(_row, index) => player.playQueue(data.tracks.data, index)}
		/>
	</div>
{/if}
