import type { TrackResponse } from '$lib/bindings/response/track/track-response';
import type { ColumnDef } from '@tanstack/table-core';
import TrackActions from './track-actions.svelte';
import TrackQuality from './track-quality.svelte';
import { renderComponent, renderSnippet } from '../ui/data-table';
import { Checkbox } from '../ui/checkbox';
import { cn } from 'tailwind-variants';
import AlbumCell from './album-cell.svelte';
import { createRawSnippet } from 'svelte';
import { formatMilliseconds } from '$lib/utils/date';

export const tracksColumns: ColumnDef<TrackResponse>[] = [
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
	{ id: 'trackNo', accessorKey: 'trackNo', header: '#' },
	{
		accessorKey: 'title',
		header: 'TITLE',
		size: 99999,
		meta: {
			mainColumn: true,
			class: cn('font-semibold'),
		},
	},
	{
		id: 'album',
		header: 'ALBUM',
		maxSize: 200,
		cell: ({ row }) => {
			if (!row.original.album) return null;
			return renderComponent(AlbumCell, { album: row.original.album });
		},
	},
	{
		accessorKey: 'artist',
		header: 'ARTIST',
		maxSize: 200,
		cell: ({ row }) => {
			if (!row.original.artist) return null;
			const anchorSnippet = createRawSnippet<[{ content: string; href: string }]>((getData) => {
				const { content, href } = getData();
				return {
					render: () => {
						return `<a href="${href}">${content}</a>`;
					},
				};
			});
			return renderSnippet(anchorSnippet, {
				content: row.original.artist.name,
				href: `/artists/${row.original.artist.id}`,
			});
		},
	},
	{
		id: 'quality',
		header: 'QUALITY',
		maxSize: 120,
		cell: ({ row }) => {
			return renderComponent(TrackQuality, { track: row.original });
		},
	},
	{
		accessorKey: 'durationMs',
		header: 'TIME',
		cell: ({ row }) => {
			return formatMilliseconds(row.original.durationMs);
		},
	},
	{
		id: 'actions',
		size: 50,
		meta: {
			class: cn('flex justify-end pr-2'),
		},
		cell: ({ row }) => {
			return renderComponent(TrackActions, { id: row.original.id });
		},
	},
];
