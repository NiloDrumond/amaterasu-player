import type { TagResponse } from '$lib/bindings/response/tag/tag-response';
import type { ColumnDef } from '@tanstack/table-core';
import TagActions from './tag-actions.svelte';
import TagSwatch from './tag-swatch.svelte';
import { renderComponent } from '../ui/data-table';
import { cn } from 'tailwind-variants';

export function tagsColumns(
	onEdit: (tag: TagResponse) => void,
	onDeleted: () => void,
): ColumnDef<TagResponse>[] {
	return [
		{
			id: 'swatch',
			size: 40,
			header: '',
			cell: ({ row }) => renderComponent(TagSwatch, { color: row.original.color ?? null }),
		},
		{
			id: 'name',
			header: 'NAME',
			size: 99999,
			meta: {
				mainColumn: true,
				class: cn('font-semibold'),
			},
			cell: ({ row }) => row.original.name,
		},
		{
			id: 'category',
			header: 'CATEGORY',
			maxSize: 160,
			cell: ({ row }) => row.original.category ?? '—',
		},
		{
			id: 'trackCount',
			header: 'TRACKS',
			maxSize: 80,
			cell: ({ row }) => String(row.original.trackCount),
		},
		{
			id: 'albumCount',
			header: 'ALBUMS',
			maxSize: 80,
			cell: ({ row }) => String(row.original.albumCount),
		},
		{
			id: 'actions',
			size: 50,
			meta: {
				class: cn('flex justify-end pr-2'),
			},
			cell: ({ row }) =>
				renderComponent(TagActions, {
					tag: row.original,
					onEdit,
					onDeleted,
				}),
		},
	];
}
