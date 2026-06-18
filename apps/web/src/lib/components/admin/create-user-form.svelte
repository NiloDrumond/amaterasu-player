<script lang="ts">
	import { Button } from '$lib/components/ui/button/index';
	import { FieldGroup, Field, FieldLabel } from '$lib/components/ui/field/index';
	import { Input } from '$lib/components/ui/input/index';
	import { createUser } from '$lib/services/admin-service';
	import { toast } from 'svelte-sonner';

	let { onCreated }: { onCreated?: () => void } = $props();

	const id = $props.id();

	let name = $state('');
	let email = $state('');
	let password = $state('');
	let submitting = $state(false);

	function generatePassword() {
		const charset = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*';
		const length = 16;
		const values = new Uint32Array(length);
		crypto.getRandomValues(values);
		password = Array.from(values, (v) => charset[v % charset.length]).join('');
	}

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		submitting = true;
		const { error } = await createUser(fetch, { name, email, password });
		submitting = false;
		if (error) {
			toast.error('Failed to create user', { description: error });
			return;
		}
		toast.success('User created', { description: email });
		name = '';
		email = '';
		password = '';
		onCreated?.();
	}
</script>

<form onsubmit={handleSubmit}>
	<FieldGroup>
		<Field>
			<FieldLabel for="name-{id}">Name</FieldLabel>
			<Input id="name-{id}" bind:value={name} type="text" required />
		</Field>
		<Field>
			<FieldLabel for="email-{id}">Email</FieldLabel>
			<Input id="email-{id}" bind:value={email} type="email" placeholder="m@example.com" required />
		</Field>
		<Field>
			<FieldLabel for="password-{id}">Password</FieldLabel>
			<div class="flex gap-2">
				<Input id="password-{id}" bind:value={password} type="text" minlength={6} required />
				<Button type="button" variant="outline" onclick={generatePassword}>Generate</Button>
			</div>
		</Field>
		<Field>
			<Button type="submit" disabled={submitting}>Create user</Button>
		</Field>
	</FieldGroup>
</form>
