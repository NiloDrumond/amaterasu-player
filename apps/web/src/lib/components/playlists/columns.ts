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
			meta: {
				mainColumn: true,
				class: cn('font-semibold'),
			},
			cell: ({ row }) => row.original.name,
		},
		{
			id: 'type',
			header: 'TYPE',
			maxSize: 90,
			cell: ({ row }) => (row.original.playlistType === 'dynamic' ? '✨ Dynamic' : 'Manual'),
		},
		{
			id: 'trackCount',
			header: 'TRACKS',
			maxSize: 80,
			cell: ({ row }) => String(row.original.trackCount),
		},
		{
			id: 'duration',
			header: 'DURATION',
			maxSize: 120,
			cell: ({ row }) => formatMilliseconds(Number(row.original.totalDurationMs)),
		},
		{
			id: 'createdAt',
			header: 'CREATED',
			maxSize: 140,
			cell: ({ row }) => new Date(row.original.createdAt).toLocaleDateString(),
		},
		{
			id: 'actions',
			size: 50,
			meta: {
				class: cn('flex justify-end pr-2'),
			},
			cell: ({ row }) => renderComponent(PlaylistActions, { id: row.original.id, onDeleted }),
		},
	];
}
