<script lang="ts">
	import { usersColumns } from '$lib/components/admin/users/columns.js';
	import CreateUserDialog from '$lib/components/admin/users/create-user-dialog.svelte';
	import DataTable from '$lib/components/ui/data-table/data-table.svelte';
	import { Button } from '$lib/components/ui/button/index.js';
	import { goto } from '$app/navigation';
	import { page } from '$app/state';

	let { data } = $props();
	let createOpen = $state(false);

	function onChangePage(newPage: number) {
		if (!data.users) return;
		const url = new URL(page.url);
		url.searchParams.set('offset', (data.users.limit * (newPage - 1)).toString());
		goto(url);
	}
</script>

{#if data.error}
	<h1>Error</h1>
	<p>{data.error.message}</p>
{:else}
	<div class="flex flex-col gap-2 p-4">
		<div class="flex items-center justify-between">
			<h1 class="tracking-widest uppercase">Users</h1>
			<Button onclick={() => (createOpen = true)}>New user</Button>
		</div>
		<DataTable
			storageKey="admin:users"
			data={data.users.data}
			columns={usersColumns}
			pagination={{
				page: data.page,
				totalPages: Math.ceil(Number(data.users.total) / data.users.limit),
				onChangePage,
			}}
		/>
	</div>
	<CreateUserDialog bind:open={createOpen} />
{/if}
