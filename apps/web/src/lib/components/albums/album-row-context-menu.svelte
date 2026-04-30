<script lang="ts">
	import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
	import TagPickerDialog from '$lib/components/tags/tag-picker-dialog.svelte';
	import type { Snippet } from 'svelte';
	import AlbumActionItems from './album-action-items.svelte';

	let {
		id,
		trigger,
	}: {
		id: string;
		trigger: Snippet<[{ props: Record<string, unknown> }]>;
	} = $props();

	let tagsOpen = $state(false);
</script>

<TagPickerDialog entity="album" entityId={id} bind:open={tagsOpen} />

<ContextMenu.Root>
	<ContextMenu.Trigger>
		{#snippet child({ props })}
			{@render trigger({ props })}
		{/snippet}
	</ContextMenu.Trigger>
	<ContextMenu.Content>
		<AlbumActionItems {id} onEditTags={() => (tagsOpen = true)} />
	</ContextMenu.Content>
</ContextMenu.Root>
