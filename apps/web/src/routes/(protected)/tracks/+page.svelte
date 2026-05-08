<script lang="ts">
	import { tracksColumns } from '$lib/components/tracks/columns.js';
	import DataTable from '$lib/components/ui/data-table/data-table.svelte';
	import TrackRowContextMenu from '$lib/components/tracks/track-row-context-menu.svelte';
	import FilterBar from '$lib/components/filters/filter-bar.svelte';
	import SearchInput from '$lib/components/filters/search-input.svelte';
	import SaveAsDynamicPlaylist from '$lib/components/filters/save-as-dynamic-playlist.svelte';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { getPlayer } from '$lib/player/player.svelte';
	import { decodeFilter, encodeFilter, getTextSearch, setTextSearch } from '$lib/utils/filter-url';
	import { applySortToUrl, extractSortFromUrl } from '$lib/utils/pagination';
	import type { FilterNode } from '$lib/bindings/filter/filter-node';
	import type { SortDir } from '$lib/bindings/request/common/sort-dir';

	let { data } = $props();
	const player = getPlayer();

	// Optimistic local state — chips render from this immediately. URL syncs in
	// the background, and we re-sync from URL when the user navigates back/forward.
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
		if (!data.tracks) return;
		const url = new URL(page.url);
		url.searchParams.set('offset', (data.tracks.limit * (newPage - 1)).toString());
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
		goto(url, { keepFocus: true, replaceState: false, noScroll: true });
	}
</script>

{#if data.error}
	<h1>Error</h1>
	<p>{data.error.message}</p>
{:else}
	<div class="flex flex-col gap-3 p-4">
		<h1 class="tracking-widest uppercase">Tracks</h1>
		<div class="flex flex-row flex-wrap items-center gap-2">
			<SearchInput
				value={getTextSearch(filter)}
				onChange={(q) => onFilterChange(setTextSearch(filter, q))}
				placeholder="Search tracks…"
			/>
			<FilterBar entity="tracks" {filter} onChange={onFilterChange} />
			<div class="ml-auto"></div>
			<SaveAsDynamicPlaylist {filter} />
		</div>
		<DataTable
			data={data.tracks.data}
			columns={tracksColumns.filter((col) => col.id !== 'trackNo')}
			pagination={{
				page: data.page,
				totalPages: Math.ceil(Number(data.tracks.total) / data.tracks.limit),
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
			onRowClick={(_row, index) => player.playQueue(data.tracks.data, index)}
		>
			{#snippet rowContextMenu({ row, trigger })}
				<TrackRowContextMenu track={row} {trigger} />
			{/snippet}
		</DataTable>
	</div>
{/if}
