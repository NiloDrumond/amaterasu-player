<script lang="ts" generics="TData">
	import type { Column, Table } from '@tanstack/table-core';
	import { Icons } from '$lib/components/ui/icons';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';

	let { table }: { table: Table<TData> } = $props();

	function labelFor(column: Column<TData, unknown>): string {
		const def = column.columnDef as { header?: unknown; meta?: { label?: string } };
		if (def.meta?.label) return def.meta.label;
		if (typeof def.header === 'string') return def.header;
		return column.id;
	}
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger>
		{#snippet child({ props })}
			<Button {...props} variant="ghost" size="icon" class="relative size-8 p-0">
				<span class="sr-only">Toggle columns</span>
				<Icons.More />
			</Button>
		{/snippet}
	</DropdownMenu.Trigger>
	<DropdownMenu.Content align="end">
		<DropdownMenu.Label>Toggle columns</DropdownMenu.Label>
		{#each table.getAllLeafColumns() as column (column.id)}
			{#if column.getCanHide()}
				<DropdownMenu.CheckboxItem
					checked={column.getIsVisible()}
					onCheckedChange={(value) => column.toggleVisibility(!!value)}
					closeOnSelect={false}
				>
					{labelFor(column)}
				</DropdownMenu.CheckboxItem>
			{/if}
		{/each}
	</DropdownMenu.Content>
</DropdownMenu.Root>
