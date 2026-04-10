<script lang="ts">
	import { enhance } from '$app/forms';
	import { Button } from '$lib/components/ui/button/index';
	import * as Card from '$lib/components/ui/card/index';
	import { FieldGroup, Field, FieldLabel, FieldError } from '$lib/components/ui/field/index';
	import { Input } from '$lib/components/ui/input/index';
	import { cn } from '$lib/utils';
	import type { HTMLAttributes } from 'svelte/elements';

	let {
		class: className,
		form,
		...restProps
	}: HTMLAttributes<HTMLDivElement> & {
		form?: { error?: string } | null;
	} = $props();

	const id = $props.id();
</script>

<div class={cn('flex flex-col gap-6', className)} {...restProps}>
	<Card.Root>
		<Card.Header class="text-center">
			<Card.Title class="text-xl">Login</Card.Title>
		</Card.Header>
		<Card.Content>
			<form method="POST" use:enhance>
				<FieldGroup>
					{#if form?.error}
						<FieldError errors={[{ message: form.error }]} />
					{/if}
					<Field>
						<FieldLabel for="email-{id}">Email</FieldLabel>
						<Input id="email-{id}" name="email" type="email" placeholder="m@example.com" required />
					</Field>
					<Field>
						<div class="flex items-center">
							<FieldLabel for="password-{id}">Password</FieldLabel>
						</div>
						<Input id="password-{id}" name="password" type="password" required />
					</Field>
					<Field>
						<Button type="submit">Login</Button>
					</Field>
				</FieldGroup>
			</form>
		</Card.Content>
	</Card.Root>
</div>
