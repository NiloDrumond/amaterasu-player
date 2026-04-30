<script lang="ts">
	import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
	import type { TagResponse } from '$lib/bindings/response/tag/tag-response';
	import type { Snippet } from 'svelte';
	import TagActionItems from './tag-action-items.svelte';

	let {
		tag,
		onEdit,
		onDeleted,
		trigger,
	}: {
		tag: TagResponse;
		onEdit: (tag: TagResponse) => void;
		onDeleted?: () => void;
		trigger: Snippet<[{ props: Record<string, unknown> }]>;
	} = $props();
</script>

<ContextMenu.Root>
	<ContextMenu.Trigger>
		{#snippet child({ props })}
			{@render trigger({ props })}
		{/snippet}
	</ContextMenu.Trigger>
	<ContextMenu.Content>
		<TagActionItems {tag} {onEdit} {onDeleted} />
	</ContextMenu.Content>
</ContextMenu.Root>
