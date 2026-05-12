import type { ArtistResponse } from '$lib/bindings/response/artist/artist-response';
import type { ColumnDef } from '@tanstack/table-core';
import ArtistActions from './artist-actions.svelte';
import { renderComponent, renderSnippet } from '../ui/data-table';
import { Checkbox } from '../ui/checkbox';
import { cn } from 'tailwind-variants';
import { createRawSnippet } from 'svelte';
import { page } from '$app/state';

export const artistsColumns: ColumnDef<ArtistResponse>[] = [
	{
		id: 'select',
		minSize: 50,
		size: 50,
		header: ({ table }) =>
			renderComponent(Checkbox, {
				checked: table.getIsAllPageRowsSelected(),
				indeterminate: table.getIsSomePageRowsSelected() && !table.getIsAllPageRowsSelected(),
				onCheckedChange: (value) => table.toggleAllPageRowsSelected(!!value),
				'aria-label': 'Select all',
			}),
		cell: ({ row }) =>
			renderComponent(Checkbox, {
				checked: row.getIsSelected(),
				onCheckedChange: (value) => row.toggleSelected(!!value),
				class: cn('mr-2'),
				'aria-label': 'Select row',
			}),
		enableSorting: false,
		enableHiding: false,
	},
	{
		id: 'name',
		header: 'NAME',
		size: 99999,
		accessorFn: (row) => row.name,
		meta: {
			mainColumn: true,
		},
		cell: ({ row }) => {
			const anchorSnippet = createRawSnippet<[{ content: string; href: string }]>((getData) => {
				const { content, href } = getData();
				return {
					render: () => `<a href="${href}">${content}</a>`,
				};
			});
			const base = page.url.pathname.startsWith('/admin') ? '/admin/artists' : '/artists';
			return renderSnippet(anchorSnippet, {
				content: row.original.name,
				href: `${base}/${row.original.id}`,
			});
		},
	},
	{
		id: 'albumCount',
		header: 'ALBUMS',
		maxSize: 100,
		accessorFn: (row) => Number(row.albumCount),
		cell: ({ row }) => String(row.original.albumCount),
	},
	{
		id: 'trackCount',
		header: 'TRACKS',
		maxSize: 100,
		accessorFn: (row) => Number(row.trackCount),
		cell: ({ row }) => String(row.original.trackCount),
	},
	{
		id: 'playCount',
		header: 'PLAYS',
		maxSize: 80,
		accessorFn: (row) => Number(row.playCount),
		cell: ({ row }) => String(row.original.playCount),
	},
	{
		id: 'actions',
		size: 50,
		meta: {
			class: cn('text-right pr-2'),
		},
		cell: ({ row }) => renderComponent(ArtistActions, { id: row.original.id }),
	},
];
