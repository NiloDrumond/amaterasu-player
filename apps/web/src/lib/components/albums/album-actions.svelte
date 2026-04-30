<script lang="ts">
	import EllipsisIcon from '@lucide/svelte/icons/ellipsis';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import TagPickerDialog from '$lib/components/tags/tag-picker-dialog.svelte';
	import AlbumActionItems from './album-action-items.svelte';

	let { id }: { id: string } = $props();

	let tagsOpen = $state(false);
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
		<AlbumActionItems {id} onEditTags={() => (tagsOpen = true)} />
	</DropdownMenu.Content>
</DropdownMenu.Root>

<TagPickerDialog entity="album" entityId={id} bind:open={tagsOpen} />
