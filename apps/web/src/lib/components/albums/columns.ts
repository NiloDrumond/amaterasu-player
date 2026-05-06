import type { AlbumResponse } from '$lib/bindings/response/album/album-response';
import type { ColumnDef } from '@tanstack/table-core';
import AlbumCoverCell from './album-cover-cell.svelte';
import AlbumActions from './album-actions.svelte';
import { renderComponent, renderSnippet } from '../ui/data-table';
import { Checkbox } from '../ui/checkbox';
import { cn } from 'tailwind-variants';
import { createRawSnippet } from 'svelte';
import { formatMilliseconds } from '$lib/utils/date';
import { page } from '$app/state';

export const albumsColumns: ColumnDef<AlbumResponse>[] = [
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
		id: 'title',
		header: 'TITLE',
		size: 99999,
		meta: {
			mainColumn: true,
		},
		cell: ({ row }) =>
			renderComponent(AlbumCoverCell, {
				id: row.original.id,
				title: row.original.title,
				coverUrl: row.original.coverUrl,
			}),
	},
	{
		id: 'artist',
		header: 'ARTIST',
		maxSize: 200,
		cell: ({ row }) => {
			if (!row.original.artist) return null;
			const anchorSnippet = createRawSnippet<[{ content: string; href: string }]>((getData) => {
				const { content, href } = getData();
				return {
					render: () => `<a href="${href}">${content}</a>`,
				};
			});
			const base = page.url.pathname.startsWith('/admin') ? '/admin/artists' : '/artists';
			return renderSnippet(anchorSnippet, {
				content: row.original.artist.name,
				href: `${base}/${row.original.artist.id}`,
			});
		},
	},
	{
		id: 'date',
		header: 'DATE',
		maxSize: 120,
		cell: ({ row }) => row.original.date ?? '',
	},
	{
		id: 'year',
		header: 'YEAR',
		maxSize: 80,
		cell: ({ row }) => {
			if (!row.original.date) return '';
			return String(new Date(row.original.date).getUTCFullYear());
		},
	},
	{
		id: 'trackCount',
		header: 'TRACKS',
		maxSize: 80,
		cell: ({ row }) => String(row.original.trackCount),
	},
	{
		id: 'time',
		header: 'TIME',
		maxSize: 100,
		cell: ({ row }) => formatMilliseconds(Number(row.original.totalDurationMs)),
	},
	{
		id: 'actions',
		size: 50,
		meta: {
			class: cn('flex justify-end pr-2'),
		},
		cell: ({ row }) => renderComponent(AlbumActions, { id: row.original.id }),
	},
];
