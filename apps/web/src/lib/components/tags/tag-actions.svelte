<script lang="ts">
	import EllipsisIcon from '@lucide/svelte/icons/ellipsis';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import type { TagResponse } from '$lib/bindings/response/tag/tag-response';
	import TagActionItems from './tag-action-items.svelte';

	let {
		tag,
		onEdit,
		onDeleted,
	}: {
		tag: TagResponse;
		onEdit: (tag: TagResponse) => void;
		onDeleted?: () => void;
	} = $props();
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
		<TagActionItems {tag} {onEdit} {onDeleted} />
	</DropdownMenu.Content>
</DropdownMenu.Root>
