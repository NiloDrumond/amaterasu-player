import type { PlaylistResponse } from '$lib/bindings/response/playlist/playlist-response';
import type { ColumnDef } from '@tanstack/table-core';
import PlaylistActions from './playlist-actions.svelte';
import { renderComponent } from '../ui/data-table';
import { cn } from 'tailwind-variants';
import { formatMilliseconds } from '$lib/utils/date';

export function playlistsColumns(onDeleted: () => void): ColumnDef<PlaylistResponse>[] {
	return [
		{
			id: 'name',
			header: 'NAME',
			size: 99999,
			enableHiding: false,
			accessorFn: (row) => row.name,
			meta: {
				mainColumn: true,
				class: cn('font-semibold'),
			},
			cell: ({ row }) => row.original.name,
		},
		{
			id: 'type',
			header: 'TYPE',
			size: 90,
			maxSize: 90,
			accessorFn: (row) => row.playlistType,
			cell: ({ row }) => (row.original.playlistType === 'dynamic' ? '✨ Dynamic' : 'Manual'),
		},
		{
			id: 'trackCount',
			header: 'TRACKS',
			size: 60,
			maxSize: 60,
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
			id: 'duration',
			header: 'DURATION',
			size: 80,
			maxSize: 80,
			accessorFn: (row) => Number(row.totalDurationMs),
			cell: ({ row }) => formatMilliseconds(Number(row.original.totalDurationMs)),
		},
		// {
		// 	id: 'createdAt',
		// 	header: 'CREATED',
		// 	maxSize: 140,
		// 	accessorFn: (row) => row.createdAt,
		// 	cell: ({ row }) => new Date(row.original.createdAt).toLocaleDateString(),
		// },
		{
			id: 'actions',
			size: 50,
			enableHiding: false,
			enableSorting: false,
			meta: {
				class: cn('text-right pr-2'),
			},
			cell: ({ row }) => renderComponent(PlaylistActions, { id: row.original.id, onDeleted }),
		},
	];
}
