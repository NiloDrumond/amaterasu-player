<script lang="ts">
	import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
	import type { Snippet } from 'svelte';
	import PlaylistActionItems from './playlist-action-items.svelte';

	let {
		id,
		onDeleted,
		trigger,
	}: {
		id: string;
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
		<PlaylistActionItems {id} {onDeleted} />
	</ContextMenu.Content>
</ContextMenu.Root>
