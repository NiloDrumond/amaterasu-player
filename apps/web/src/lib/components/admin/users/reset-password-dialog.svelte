<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Field, FieldGroup, FieldLabel } from '$lib/components/ui/field/index.js';
	import { resetUserPassword } from '$lib/services/admin-service';
	import type { AdminUserResponse } from '$lib/bindings/response/admin/admin-user-response';
	import { toast } from 'svelte-sonner';

	let { user, open = $bindable(false) }: { user: AdminUserResponse; open: boolean } = $props();

	const fieldId = $props.id();
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
		const { error } = await resetUserPassword(fetch, user.id, password);
		submitting = false;
		if (error) {
			toast.error('Failed to reset password', { description: error });
			return;
		}
		toast.success('Password reset', { description: user.email });
		open = false;
	}

	$effect(() => {
		if (!open) password = '';
	});
</script>

<Dialog.Root bind:open>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>Reset password</Dialog.Title>
			<Dialog.Description>Set a new password for {user.email}.</Dialog.Description>
		</Dialog.Header>
		<form onsubmit={handleSubmit}>
			<FieldGroup>
				<Field>
					<FieldLabel for="reset-pw-{fieldId}">New password</FieldLabel>
					<div class="flex gap-2">
						<Input
							id="reset-pw-{fieldId}"
							bind:value={password}
							type="text"
							minlength={6}
							required
						/>
						<Button type="button" variant="outline" onclick={generatePassword}>Generate</Button>
					</div>
				</Field>
				<Dialog.Footer>
					<Dialog.Close>
						{#snippet child({ props })}
							<Button {...props} variant="ghost" type="button">Cancel</Button>
						{/snippet}
					</Dialog.Close>
					<Button type="submit" disabled={submitting || password.length < 6}>
						{submitting ? 'Resetting…' : 'Reset password'}
					</Button>
				</Dialog.Footer>
			</FieldGroup>
		</form>
	</Dialog.Content>
</Dialog.Root>
