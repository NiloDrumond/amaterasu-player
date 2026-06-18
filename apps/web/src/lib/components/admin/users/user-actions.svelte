<script lang="ts">
	import { Icons } from '$lib/components/ui/icons';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import ResetPasswordDialog from './reset-password-dialog.svelte';
	import DeleteUserDialog from './delete-user-dialog.svelte';
	import type { AdminUserResponse } from '$lib/bindings/response/admin/admin-user-response';

	let { user }: { user: AdminUserResponse } = $props();

	let resetOpen = $state(false);
	let deleteOpen = $state(false);
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger>
		{#snippet child({ props })}
			<Button {...props} variant="ghost" size="icon" class="relative size-8 p-0">
				<span class="sr-only">Open menu</span>
				<Icons.More />
			</Button>
		{/snippet}
	</DropdownMenu.Trigger>
	<DropdownMenu.Content>
		<DropdownMenu.Group>
			<DropdownMenu.Label>Actions</DropdownMenu.Label>
			<DropdownMenu.Item onclick={() => navigator.clipboard.writeText(user.id)}>
				Copy ID
			</DropdownMenu.Item>
			<DropdownMenu.Item onclick={() => (resetOpen = true)}>Reset password…</DropdownMenu.Item>
			<DropdownMenu.Separator />
			<DropdownMenu.Item variant="destructive" onclick={() => (deleteOpen = true)}>
				Delete…
			</DropdownMenu.Item>
		</DropdownMenu.Group>
	</DropdownMenu.Content>
</DropdownMenu.Root>

<ResetPasswordDialog {user} bind:open={resetOpen} />
<DeleteUserDialog {user} bind:open={deleteOpen} />
