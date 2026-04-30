<script lang="ts">
	import EllipsisIcon from '@lucide/svelte/icons/ellipsis';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import { toast } from 'svelte-sonner';
	import { deleteTag } from '$lib/services/tag-service';
	import type { TagResponse } from '$lib/bindings/response/tag/tag-response';

	let {
		tag,
		onEdit,
		onDeleted,
	}: {
		tag: TagResponse;
		onEdit: (tag: TagResponse) => void;
		onDeleted?: () => void;
	} = $props();

	async function handleDelete() {
		const { error } = await deleteTag(fetch, tag.id);
		if (error) {
			toast.error(error);
		} else {
			toast.success('Tag deleted');
			onDeleted?.();
		}
	}
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger>
		{#snippet child({ props })}
			<Button {...props} variant="ghost" size="icon" class="relative size-8 p-0">
				<span class="sr-only">Open menu</span>
				<EllipsisIcon />
			</Button>
		{/snippet}
	</DropdownMenu.Trigger>
	<DropdownMenu.Content>
		<DropdownMenu.Group>
			<DropdownMenu.Label>Actions</DropdownMenu.Label>
			<DropdownMenu.Item onclick={() => onEdit(tag)}>Edit</DropdownMenu.Item>
			<DropdownMenu.Item onclick={handleDelete} class="text-destructive focus:text-destructive">
				Delete
			</DropdownMenu.Item>
		</DropdownMenu.Group>
	</DropdownMenu.Content>
</DropdownMenu.Root>
