<script lang="ts">
	import { albumsColumns } from '$lib/components/albums/columns.js';
	import DataTable from '$lib/components/ui/data-table/data-table.svelte';
	import { getPlayer } from '$lib/player/player.svelte';
	import AlbumRowContextMenu from '$lib/components/albums/album-row-context-menu.svelte';
	import FilterBar from '$lib/components/filters/filter-bar.svelte';
	import SearchInput from '$lib/components/filters/search-input.svelte';
	import SaveAsAlbumCollection from '$lib/components/filters/save-as-album-collection.svelte';
	import { untrack } from 'svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { decodeFilter, encodeFilter, getTextSearch, setTextSearch } from '$lib/utils/filter-url';
	import { applySortToUrl, extractSortFromUrl } from '$lib/utils/pagination';
	import type { FilterNode } from '$lib/bindings/filter/filter-node';
	import type { SortDir } from '$lib/bindings/request/common/sort-dir';

	let { data } = $props();
	const player = getPlayer();

	let filter = $state<FilterNode | null>(decodeFilter(page.url.searchParams.get('f')));
	let lastSyncedF = $state(page.url.searchParams.get('f') ?? '');

	$effect(() => {
		// Resync ONLY when the URL itself changes (back/forward, external nav).
		// `lastSyncedF` is read untracked so that mutating it inside `onFilterChange`
		// before calling `goto()` does not re-fire this effect against the stale URL.
		const urlF = page.url.searchParams.get('f') ?? '';
		untrack(() => {
			if (urlF !== lastSyncedF) {
				filter = decodeFilter(urlF);
				lastSyncedF = urlF;
			}
		});
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
		<FilterBar entity="albums" {filter} onChange={onFilterChange}>
			{#snippet leading()}
				<SearchInput
					value={getTextSearch(filter)}
					onChange={(q) => onFilterChange(setTextSearch(filter, q))}
					placeholder="Search albums…"
				/>
			{/snippet}
			{#snippet trailing()}
				<SaveAsAlbumCollection {filter} />
			{/snippet}
		</FilterBar>
		<DataTable
			storageKey="albums"
			data={data.albums.data}
			columns={albumsColumns}
			pagination={{
				page: data.page,
				totalPages: Math.ceil(Number(data.albums.total) / data.albums.limit),
				onChangePage: onChangePage,
			}}
			serverSort={{
				...extractSortFromUrl(page.url),
				onSortChange: (sort: string | null, dir: SortDir) => {
					goto(applySortToUrl(new URL(page.url), sort, dir), {
						keepFocus: true,
						noScroll: true,
					});
				},
			}}
			onRowClick={(row) => goto(`/albums/${row.id}`)}
			isRowPlaying={(row) => row.id === player.currentOrigin.albumId}
		>
			{#snippet rowContextMenu({ row, trigger })}
				<AlbumRowContextMenu id={row.id} {trigger} />
			{/snippet}
		</DataTable>
	</div>
{/if}
