<script lang="ts">
	import { artistsColumns } from '$lib/components/artists/columns.js';
	import DataTable from '$lib/components/ui/data-table/data-table.svelte';
	import ArtistRowContextMenu from '$lib/components/artists/artist-row-context-menu.svelte';
	import SearchInput from '$lib/components/filters/search-input.svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';

	let { data } = $props();

	function onChangePage(newPage: number) {
		if (!data.artists) return;
		const url = new URL(page.url);
		url.searchParams.set('offset', (data.artists.limit * (newPage - 1)).toString());
		goto(url);
	}

	function onSearch(q: string) {
		const url = new URL(page.url);
		if (q) url.searchParams.set('q', q);
		else url.searchParams.delete('q');
		url.searchParams.delete('offset');
		goto(url, { keepFocus: true, noScroll: true });
	}
</script>

{#if data.error}
	<h1>Error</h1>
	<p>{data.error.message}</p>
{:else}
	<div class="flex flex-col gap-3 p-4">
		<h1 class="tracking-widest uppercase">Artists</h1>
		<SearchInput
			value={page.url.searchParams.get('q') ?? ''}
			onChange={onSearch}
			placeholder="Search artists…"
		/>
		<DataTable
			data={data.artists.data}
			columns={artistsColumns}
			pagination={{
				page: data.page,
				totalPages: Math.ceil(Number(data.artists.total) / data.artists.limit),
				onChangePage: onChangePage,
			}}
			onRowClick={(row) => goto(`/artists/${row.id}`)}
		>
			{#snippet rowContextMenu({ row, trigger })}
				<ArtistRowContextMenu id={row.id} {trigger} />
			{/snippet}
		</DataTable>
	</div>
{/if}
