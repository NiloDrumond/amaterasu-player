import type { AdminUserResponse } from '$lib/bindings/response/admin/admin-user-response';
import type { ColumnDef } from '@tanstack/table-core';
import UserActions from './user-actions.svelte';
import UserRoleBadge from './user-role-badge.svelte';
import { renderComponent } from '../../ui/data-table';
import { cn } from 'tailwind-variants';

export const usersColumns: ColumnDef<AdminUserResponse>[] = [
	{
		id: 'name',
		header: 'NAME',
		size: 99999,
		enableHiding: false,
		accessorFn: (row) => row.name,
		meta: {
			mainColumn: true,
		},
		cell: ({ row }) => row.original.name,
	},
	{
		id: 'email',
		header: 'EMAIL',
		accessorFn: (row) => row.email,
		cell: ({ row }) => row.original.email,
	},
	{
		id: 'role',
		header: 'ROLE',
		maxSize: 100,
		accessorFn: (row) => row.role,
		cell: ({ row }) => renderComponent(UserRoleBadge, { role: row.original.role }),
	},
	{
		id: 'createdAt',
		header: 'CREATED',
		maxSize: 140,
		accessorFn: (row) => row.createdAt,
		cell: ({ row }) => new Date(row.original.createdAt).toLocaleDateString(),
	},
	{
		id: 'actions',
		size: 50,
		enableHiding: false,
		enableSorting: false,
		meta: {
			class: cn('text-right pr-2'),
		},
		cell: ({ row }) => renderComponent(UserActions, { user: row.original }),
	},
];
