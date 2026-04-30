<script lang="ts">
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

<DropdownMenu.Group>
	<DropdownMenu.Label>Actions</DropdownMenu.Label>
	<DropdownMenu.Item onclick={() => onEdit(tag)}>Edit</DropdownMenu.Item>
	<DropdownMenu.Item onclick={handleDelete} class="text-destructive focus:text-destructive">
		Delete
	</DropdownMenu.Item>
</DropdownMenu.Group>
