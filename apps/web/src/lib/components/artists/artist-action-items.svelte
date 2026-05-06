<script lang="ts">
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import { page } from '$app/state';

	let { id }: { id: string } = $props();
	const isAdmin = $derived(page.data.user?.role === 'admin');
</script>

<DropdownMenu.Group>
	<DropdownMenu.Label>Actions</DropdownMenu.Label>
	<DropdownMenu.Item onclick={() => navigator.clipboard.writeText(id)}>Copy ID</DropdownMenu.Item>
	{#if isAdmin}
		<DropdownMenu.Separator />
		<DropdownMenu.Item onclick={() => window.open(`/admin/artists/${id}`, '_blank', 'noopener')}>
			Edit (admin)
		</DropdownMenu.Item>
	{/if}
</DropdownMenu.Group>
