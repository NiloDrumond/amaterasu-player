<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import CreateUserForm from '../create-user-form.svelte';
	import { invalidateAll } from '$app/navigation';

	let { open = $bindable(false) }: { open: boolean } = $props();

	async function handleCreated() {
		open = false;
		await invalidateAll();
	}
</script>

<Dialog.Root bind:open>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>New user</Dialog.Title>
			<Dialog.Description>They'll sign in with the email and password you set.</Dialog.Description>
		</Dialog.Header>
		<CreateUserForm onCreated={handleCreated} />
	</Dialog.Content>
</Dialog.Root>
