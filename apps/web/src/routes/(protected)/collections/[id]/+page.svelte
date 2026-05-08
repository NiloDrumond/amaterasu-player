<script lang="ts">
	import { invalidateAll, goto } from '$app/navigation';
	import { page } from '$app/state';
	import { albumsColumns } from '$lib/components/albums/columns.js';
	import DataTable from '$lib/components/ui/data-table/data-table.svelte';
	import AlbumRowContextMenu from '$lib/components/albums/album-row-context-menu.svelte';
	import FilterBar from '$lib/components/filters/filter-bar.svelte';
	import SearchInput from '$lib/components/filters/search-input.svelte';
	import { getTextSearch, setTextSearch } from '$lib/utils/filter-url';
	import { updateCollectionFilter } from '$lib/services/album-collection-service';
	import LayoutGridIcon from '@lucide/svelte/icons/layout-grid';
	import { toast } from 'svelte-sonner';
	import type { FilterNode } from '$lib/bindings/filter/filter-node';

	let { data } = $props();

	// Writable derived: tracks the server filter; `onFilterChange` assigns
	// optimistically before the round-trip resolves.
	let filter: FilterNode | null = $derived(data.collection.filterDefinition);

	function onChangePage(newPage: number) {
		const url = new URL(page.url);
		url.searchParams.set('offset', (data.albums.limit * (newPage - 1)).toString());
		goto(url);
	}

	async function onFilterChange(next: FilterNode | null) {
		// A collection always has a filter; clearing it would invalidate the collection.
		if (!next) return;
		filter = next;
		const { error } = await updateCollectionFilter(fetch, data.collection.id, {
			filterDefinition: next,
		});
		if (error) {
			toast.error('Failed to update filter');
			return;
		}
		await invalidateAll();
	}
</script>

<div class="flex flex-col gap-6 p-6">
	<div class="flex flex-row items-end gap-6">
		<div
			class="flex size-48 shrink-0 items-center justify-center rounded-lg bg-muted text-muted-foreground"
		>
			<LayoutGridIcon class="size-16 opacity-30" />
		</div>
		<div class="flex min-w-0 flex-col gap-1">
			<p class="text-xs font-medium tracking-widest text-muted-foreground uppercase">
				Album Collection
			</p>
			<h1 class="truncate text-3xl">{data.collection.name}</h1>
			<p class="text-sm text-muted-foreground">
				{`${String(data.albums.total)} albums`}
			</p>
		</div>
	</div>

	<div class="flex flex-col gap-2">
		<p class="text-xs tracking-widest text-muted-foreground uppercase">Filters</p>
		<div class="flex flex-row flex-wrap items-center gap-2">
			<SearchInput
				value={getTextSearch(filter)}
				onChange={(q) => onFilterChange(setTextSearch(filter, q))}
				placeholder="Search albums…"
			/>
			<FilterBar entity="albums" {filter} onChange={onFilterChange} />
		</div>
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
