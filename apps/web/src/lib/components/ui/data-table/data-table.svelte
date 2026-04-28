<script lang="ts" generics="TData, TValue">
	import {
		type ColumnDef,
		type ColumnFiltersState,
		getCoreRowModel,
		getFilteredRowModel,
		getPaginationRowModel,
		getSortedRowModel,
		type RowSelectionState,
		type VisibilityState,
	} from '@tanstack/table-core';
	import { createSvelteTable, FlexRender } from '$lib/components/ui/data-table/index.js';
	import * as Table from '$lib/components/ui/table/index.js';
	import Button from '../button/button.svelte';
	import { Icons } from '../icons';
	import { cn } from 'tailwind-variants';

	type DataTableProps<TData, TValue> = {
		columns: ColumnDef<TData, TValue>[];
		data: TData[];
		pagination?: {
			page: number;
			totalPages: number;
			onChangePage: (page: number) => void;
		};
		onRowClick?: (row: TData, index: number) => void;
	};

	let { data, columns, pagination, onRowClick }: DataTableProps<TData, TValue> = $props();

	function handleRowClick(event: MouseEvent, row: TData, index: number) {
		if (!onRowClick) return;
		const target = event.target as HTMLElement | null;
		if (target?.closest('a, button, input, label, [role="checkbox"]')) return;
		onRowClick(row, index);
	}

	let columnFilters = $state<ColumnFiltersState>([]);
	let columnVisibility = $state<VisibilityState>({});
	let rowSelection = $state<RowSelectionState>({});

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
		getCoreRowModel: getCoreRowModel(),
		getPaginationRowModel: getPaginationRowModel(),
		getSortedRowModel: getSortedRowModel(),
		getFilteredRowModel: getFilteredRowModel(),
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
		},
	});
</script>

<div class="rounded-md border">
	<Table.Root>
		<Table.Header>
			{#each table.getHeaderGroups() as headerGroup (headerGroup.id)}
				<Table.Row>
					{#each headerGroup.headers as header (header.id)}
						<Table.Head
							colspan={header.colSpan}
							style={`width: ${header.getSize()}px;`}
							class={header.column.columnDef.meta?.class}
						>
							{#if !header.isPlaceholder}
								<FlexRender
									content={header.column.columnDef.header}
									context={header.getContext()}
								/>
							{/if}
						</Table.Head>
					{/each}
				</Table.Row>
			{/each}
		</Table.Header>
		<Table.Body>
			{#each table.getRowModel().rows as row (row.id)}
				<Table.Row
					data-state={row.getIsSelected() && 'selected'}
					class={onRowClick ? 'cursor-pointer' : undefined}
					onclick={onRowClick
						? (e: MouseEvent) => handleRowClick(e, row.original, row.index)
						: undefined}
				>
					{#each row.getVisibleCells() as cell (cell.id)}
						<Table.Cell class={cell.column.columnDef.meta?.class}>
							<FlexRender content={cell.column.columnDef.cell} context={cell.getContext()} />
						</Table.Cell>
					{/each}
				</Table.Row>
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
				disabled={pagination.page - 1 <= 0}
				onclick={() => pagination.onChangePage(pagination.page - 1)}
			>
				<Icons.GoLeft />
			</Button>
			{#if pagination.page - 1 > 1}
				<Button onclick={() => pagination.onChangePage(pagination.page - 2)}>
					{pagination.page - 2}
				</Button>
			{/if}
			{#if pagination.page > 1}
				<Button onclick={() => pagination.onChangePage(pagination.page - 1)}>
					{pagination.page - 1}
				</Button>
			{/if}
			<div>
				<Button disabled>
					{pagination.page}
				</Button>
			</div>
			{#if pagination.page < pagination.totalPages}
				<Button onclick={() => pagination.onChangePage(pagination.page + 1)}>
					{pagination.page + 1}
				</Button>
			{/if}
			{#if pagination.page + 1 < pagination.totalPages}
				<Button onclick={() => pagination.onChangePage(pagination.page + 2)}>
					{pagination.page + 2}
				</Button>
			{/if}
			{#if pagination.page + 2 < pagination.totalPages}
				<Button onclick={() => pagination.onChangePage(pagination.page + 3)}>
					{pagination.page + 3}
				</Button>
			{/if}
			<Button
				disabled={pagination.page + 1 > pagination.totalPages}
				onclick={() => pagination.onChangePage(pagination.page + 1)}
			>
				<Icons.GoRight />
			</Button>
		</div>
	{/if}
</div>
