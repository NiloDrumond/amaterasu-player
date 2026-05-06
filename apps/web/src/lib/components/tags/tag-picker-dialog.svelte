<script lang="ts">
	import * as Dialog from '$lib/components/ui/dialog/index.js';
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
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>Tags</Dialog.Title>
			<Dialog.Description>Add or remove tags for this {entity}.</Dialog.Description>
		</Dialog.Header>

		{#if open}
			<TagPicker {entity} {entityId} />
		{/if}

		<Dialog.Footer>
			<Dialog.Close>
				{#snippet child({ props })}
					<Button {...props} variant="ghost" type="button">Done</Button>
				{/snippet}
			</Dialog.Close>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
