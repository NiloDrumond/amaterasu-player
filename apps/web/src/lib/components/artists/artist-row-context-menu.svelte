<script lang="ts">
	import * as ContextMenu from '$lib/components/ui/context-menu/index.js';
	import type { Snippet } from 'svelte';
	import ArtistActionItems from './artist-action-items.svelte';

	let {
		id,
		trigger,
	}: {
		id: string;
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
		<ArtistActionItems {id} />
	</ContextMenu.Content>
</ContextMenu.Root>
