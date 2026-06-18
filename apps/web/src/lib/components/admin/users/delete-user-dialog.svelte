<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { deleteAdminUser } from '$lib/services/admin-service';
	import type { AdminUserResponse } from '$lib/bindings/response/admin/admin-user-response';
	import { invalidateAll } from '$app/navigation';
	import { toast } from 'svelte-sonner';

	let { user, open = $bindable(false) }: { user: AdminUserResponse; open: boolean } = $props();

	let submitting = $state(false);

	async function handleDelete() {
		submitting = true;
		const { error } = await deleteAdminUser(fetch, user.id);
		submitting = false;
		if (error) {
			toast.error('Failed to delete user', { description: error });
			return;
		}
		toast.success('User deleted', { description: user.email });
		open = false;
		await invalidateAll();
	}
</script>

<Dialog.Root bind:open>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>Delete user</Dialog.Title>
			<Dialog.Description>
				Permanently delete {user.email}? This cannot be undone.
			</Dialog.Description>
		</Dialog.Header>
		<Dialog.Footer>
			<Dialog.Close>
				{#snippet child({ props })}
					<Button {...props} variant="ghost" type="button">Cancel</Button>
				{/snippet}
			</Dialog.Close>
			<Button variant="destructive" onclick={handleDelete} disabled={submitting}>
				{submitting ? 'Deleting…' : 'Delete'}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
