<script lang="ts" generics="TData, TValue">
	import {
		type ColumnDef,
		type ColumnFiltersState,
		getCoreRowModel,
		getFilteredRowModel,
		getPaginationRowModel,
		getSortedRowModel,
		type RowSelectionState,
		type SortingState,
		type VisibilityState,
	} from '@tanstack/table-core';
	import { createSvelteTable, FlexRender } from '$lib/components/ui/data-table/index.js';
	import * as Table from '$lib/components/ui/table/index.js';
	import Button from '../button/button.svelte';
	import { Icons } from '../icons';
	import { cn } from 'tailwind-variants';
	import type { Snippet } from 'svelte';
	import type { SortDir } from '$lib/bindings/request/common/sort-dir';

	type RowTrigger = Snippet<[{ props: Record<string, unknown> }]>;

	type ServerSort = {
		sort: string | null;
		dir: SortDir;
		onSortChange: (sort: string | null, dir: SortDir) => void;
	};

	type DataTableProps<TData, TValue> = {
		columns: ColumnDef<TData, TValue>[];
		data: TData[];
		pagination?: {
			page: number;
			totalPages: number;
			onChangePage: (page: number) => void;
		};
		serverSort?: ServerSort;
		onRowClick?: (row: TData, index: number) => void;
		rowContextMenu?: Snippet<[{ row: TData; trigger: RowTrigger }]>;
		rowSelection?: RowSelectionState;
		onSelectionChange?: (rows: TData[]) => void;
	};

	let {
		data,
		columns,
		pagination,
		serverSort,
		onRowClick,
		rowContextMenu,
		rowSelection = $bindable<RowSelectionState>({}),
		onSelectionChange,
	}: DataTableProps<TData, TValue> = $props();

	function handleRowClick(event: MouseEvent, row: TData, index: number) {
		if (!onRowClick) return;
		const target = event.target as HTMLElement | null;
		if (target?.closest('a, button, input, label, [role="checkbox"]')) return;
		onRowClick(row, index);
	}

	let columnFilters = $state<ColumnFiltersState>([]);
	let columnVisibility = $state<VisibilityState>({});

	const sorting = $derived<SortingState>(
		serverSort?.sort ? [{ id: serverSort.sort, desc: serverSort.dir === 'desc' }] : [],
	);

	const table = createSvelteTable({
		get data() {
			return data;
		},
		get columns() {
			return columns;
		},
		defaultColumn: {
			size: 50,
		},
		rowCount: 32,
		get manualSorting() {
			return !!serverSort;
		},
		enableSortingRemoval: true,
		getCoreRowModel: getCoreRowModel(),
		getPaginationRowModel: getPaginationRowModel(),
		get getSortedRowModel() {
			return serverSort ? undefined : getSortedRowModel();
		},
		getFilteredRowModel: getFilteredRowModel(),
		onSortingChange: (updater) => {
			if (!serverSort) return;
			const next = typeof updater === 'function' ? updater(sorting) : updater;
			if (next.length === 0) {
				serverSort.onSortChange(null, 'asc');
			} else {
				const s = next[0];
				serverSort.onSortChange(s.id, s.desc ? 'desc' : 'asc');
			}
		},
		onColumnFiltersChange: (updater) => {
			if (typeof updater === 'function') {
				columnFilters = updater(columnFilters);
			} else {
				columnFilters = updater;
			}
		},
		onColumnVisibilityChange: (updater) => {
			if (typeof updater === 'function') {
				columnVisibility = updater(columnVisibility);
			} else {
				columnVisibility = updater;
			}
		},
		onRowSelectionChange: (updater) => {
			if (typeof updater === 'function') {
				rowSelection = updater(rowSelection);
			} else {
				rowSelection = updater;
			}
			if (onSelectionChange) {
				onSelectionChange(table.getSelectedRowModel().rows.map((r) => r.original));
			}
		},
		state: {
			pagination: {
				pageSize: 32,
				pageIndex: 0,
			},
			get columnFilters() {
				return columnFilters;
			},
			get columnVisibility() {
				return columnVisibility;
			},
			get rowSelection() {
				return rowSelection;
			},
			get sorting() {
				return sorting;
			},
		},
	});
</script>

<div class="rounded-md border">
	<Table.Root>
		<Table.Header>
			{#each table.getHeaderGroups() as headerGroup (headerGroup.id)}
				<Table.Row>
					{#each headerGroup.headers as header (header.id)}
						{@const maxSize = header.column.columnDef.maxSize}
						{@const wrapStyle = maxSize ? `max-width: ${maxSize}px;` : undefined}
						<Table.Head
							colspan={header.colSpan}
							style={`width: ${header.getSize()}px;${maxSize ? ` max-width: ${maxSize}px;` : ''}`}
							class={header.column.columnDef.meta?.class}
						>
							{#if !header.isPlaceholder}
								{#if header.column.getCanSort()}
									{@const sorted = header.column.getIsSorted()}
									<button
										type="button"
										class="inline-flex cursor-pointer items-center gap-1 select-none hover:text-foreground"
										onclick={header.column.getToggleSortingHandler()}
									>
										{#if wrapStyle}
											<span
												class="overflow-hidden text-ellipsis whitespace-nowrap"
												style={wrapStyle}
											>
												<FlexRender
													content={header.column.columnDef.header}
													context={header.getContext()}
												/>
											</span>
										{:else}
											<FlexRender
												content={header.column.columnDef.header}
												context={header.getContext()}
											/>
										{/if}
										{#if sorted === 'asc'}
											<Icons.SortAsc class="size-3" />
										{:else if sorted === 'desc'}
											<Icons.SortDesc class="size-3" />
										{/if}
									</button>
								{:else if wrapStyle}
									<span
										class="block overflow-hidden text-ellipsis whitespace-nowrap"
										style={wrapStyle}
									>
										<FlexRender
											content={header.column.columnDef.header}
											context={header.getContext()}
										/>
									</span>
								{:else}
									<FlexRender
										content={header.column.columnDef.header}
										context={header.getContext()}
									/>
								{/if}
							{/if}
						</Table.Head>
					{/each}
				</Table.Row>
			{/each}
		</Table.Header>
		<Table.Body>
			{#each table.getRowModel().rows as row (row.id)}
				{#snippet rowTrigger({ props: triggerProps }: { props: Record<string, unknown> })}
					<Table.Row
						{...triggerProps}
						data-state={row.getIsSelected() ? 'selected' : undefined}
						class={onRowClick ? 'cursor-pointer' : undefined}
						onclick={onRowClick
							? (e: MouseEvent) => handleRowClick(e, row.original, row.index)
							: undefined}
					>
						{#each row.getVisibleCells() as cell (cell.id)}
							{@const cellMax = cell.column.columnDef.maxSize}
							<Table.Cell
								class={cell.column.columnDef.meta?.class}
								style={cellMax ? `max-width: ${cellMax}px;` : undefined}
							>
								{#if cellMax}
									<div
										class="overflow-hidden text-ellipsis whitespace-nowrap"
										style={`max-width: ${cellMax}px;`}
									>
										<FlexRender content={cell.column.columnDef.cell} context={cell.getContext()} />
									</div>
								{:else}
									<FlexRender content={cell.column.columnDef.cell} context={cell.getContext()} />
								{/if}
							</Table.Cell>
						{/each}
					</Table.Row>
				{/snippet}
				{#if rowContextMenu}
					{@render rowContextMenu({ row: row.original, trigger: rowTrigger })}
				{:else}
					{@render rowTrigger({ props: {} })}
				{/if}
			{:else}
				<Table.Row>
					<Table.Cell colspan={columns.length} class="h-24 text-center">No results.</Table.Cell>
				</Table.Row>
			{/each}
		</Table.Body>
	</Table.Root>
	{#if pagination}
		<div class={cn('flex flex-row items-center justify-center gap-2 border-t font-medium')}>
			<Button
				variant="ghost"
				disabled={pagination.page - 1 <= 0}
				onclick={() => pagination.onChangePage(pagination.page - 1)}
			>
				<Icons.GoLeft />
			</Button>
			{#if pagination.page - 1 > 1}
				<Button variant="ghost" onclick={() => pagination.onChangePage(pagination.page - 2)}>
					{pagination.page - 2}
				</Button>
			{/if}
			{#if pagination.page > 1}
				<Button variant="ghost" onclick={() => pagination.onChangePage(pagination.page - 1)}>
					{pagination.page - 1}
				</Button>
			{/if}
			<div>
				<Button variant="ghost" disabled>
					{pagination.page}
				</Button>
			</div>
			{#if pagination.page < pagination.totalPages}
				<Button variant="ghost" onclick={() => pagination.onChangePage(pagination.page + 1)}>
					{pagination.page + 1}
				</Button>
			{/if}
			{#if pagination.page + 1 < pagination.totalPages}
				<Button variant="ghost" onclick={() => pagination.onChangePage(pagination.page + 2)}>
					{pagination.page + 2}
				</Button>
			{/if}
			{#if pagination.page + 2 < pagination.totalPages}
				<Button variant="ghost" onclick={() => pagination.onChangePage(pagination.page + 3)}>
					{pagination.page + 3}
				</Button>
			{/if}
			<Button
				variant="ghost"
				disabled={pagination.page + 1 > pagination.totalPages}
				onclick={() => pagination.onChangePage(pagination.page + 1)}
			>
				<Icons.GoRight />
			</Button>
		</div>
	{/if}
</div>
