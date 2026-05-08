<script lang="ts">
	import { albumsColumns } from '$lib/components/albums/columns.js';
	import DataTable from '$lib/components/ui/data-table/data-table.svelte';
	import AlbumRowContextMenu from '$lib/components/albums/album-row-context-menu.svelte';
	import FilterBar from '$lib/components/filters/filter-bar.svelte';
	import SearchInput from '$lib/components/filters/search-input.svelte';
	import SaveAsAlbumCollection from '$lib/components/filters/save-as-album-collection.svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { decodeFilter, encodeFilter, getTextSearch, setTextSearch } from '$lib/utils/filter-url';
	import type { FilterNode } from '$lib/bindings/filter/filter-node';

	let { data } = $props();

	let filter = $state<FilterNode | null>(decodeFilter(page.url.searchParams.get('f')));
	let lastSyncedF = $state(page.url.searchParams.get('f') ?? '');

	$effect(() => {
		const urlF = page.url.searchParams.get('f') ?? '';
		if (urlF !== lastSyncedF) {
			filter = decodeFilter(urlF);
			lastSyncedF = urlF;
		}
	});

	function onChangePage(newPage: number) {
		if (!data.albums) return;
		const url = new URL(page.url);
		url.searchParams.set('offset', (data.albums.limit * (newPage - 1)).toString());
		goto(url);
	}

	function onFilterChange(next: FilterNode | null) {
		filter = next;
		const url = new URL(page.url);
		const encoded = encodeFilter(next);
		if (encoded) url.searchParams.set('f', encoded);
		else url.searchParams.delete('f');
		url.searchParams.delete('offset');
		lastSyncedF = encoded ?? '';
		goto(url, { keepFocus: true, noScroll: true });
	}
</script>

{#if data.error}
	<h1>Error</h1>
	<p>{data.error.message}</p>
{:else}
	<div class="flex flex-col gap-3 p-4">
		<h1 class="tracking-widest uppercase">Albums</h1>
		<div class="flex flex-row flex-wrap items-center gap-2">
			<SearchInput
				value={getTextSearch(filter)}
				onChange={(q) => onFilterChange(setTextSearch(filter, q))}
				placeholder="Search albums…"
			/>
			<FilterBar entity="albums" {filter} onChange={onFilterChange} />
			<div class="ml-auto"></div>
			<SaveAsAlbumCollection {filter} />
		</div>
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
