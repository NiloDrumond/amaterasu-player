<script lang="ts">
	import { Dialog } from 'bits-ui';
	import { Button } from '$lib/components/ui/button';
	import TagPicker from './tag-picker.svelte';

	let {
		entity,
		entityId,
		open = $bindable(false),
		onClose,
	}: {
		entity: 'track' | 'album';
		entityId: string;
		open: boolean;
		onClose?: () => void;
	} = $props();

	$effect(() => {
		if (!open) onClose?.();
	});
</script>

<Dialog.Root bind:open>
	<Dialog.Portal>
		<Dialog.Overlay class="fixed inset-0 z-50 bg-black/50" />
		<Dialog.Content
			class="fixed top-1/2 left-1/2 z-50 w-full max-w-md -translate-x-1/2 -translate-y-1/2 rounded-lg bg-background p-6 shadow-lg"
		>
			<Dialog.Title class="mb-1 text-lg font-semibold">Tags</Dialog.Title>
			<Dialog.Description class="mb-4 text-sm text-muted-foreground">
				Add or remove tags for this {entity}.
			</Dialog.Description>

			{#if open}
				<TagPicker {entity} {entityId} />
			{/if}

			<div class="mt-6 flex justify-end">
				<Dialog.Close>
					{#snippet child({ props })}
						<Button {...props} variant="ghost" type="button">Done</Button>
					{/snippet}
				</Dialog.Close>
			</div>
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>
