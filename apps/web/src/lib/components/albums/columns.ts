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
		enableHiding: false,
		accessorFn: (row) => row.title,
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
		size: 150,
		maxSize: 150,
		accessorFn: (row) => row.artist?.name ?? '',
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
		id: 'year',
		header: 'YEAR',
		maxSize: 80,
		accessorFn: (row) => row.date ?? '',
		cell: ({ row }) => {
			if (!row.original.date) return '';
			return String(new Date(row.original.date).getUTCFullYear());
		},
	},
	{
		id: 'trackCount',
		header: 'TRACKS',
		maxSize: 80,
		accessorFn: (row) => Number(row.trackCount),
		cell: ({ row }) => String(row.original.trackCount),
	},
	{
		id: 'playCount',
		header: 'PLAYS',
		size: 60,
		maxSize: 60,
		accessorFn: (row) => Number(row.playCount),
		cell: ({ row }) => String(row.original.playCount),
	},
	{
		id: 'time',
		header: 'TIME',
		size: 80,
		maxSize: 80,
		accessorFn: (row) => Number(row.totalDurationMs),
		cell: ({ row }) => formatMilliseconds(Number(row.original.totalDurationMs)),
	},
	{
		id: 'actions',
		size: 50,
		enableHiding: false,
		enableSorting: false,
		meta: {
			class: cn('text-right pr-2'),
		},
		cell: ({ row }) => renderComponent(AlbumActions, { id: row.original.id }),
	},
];
