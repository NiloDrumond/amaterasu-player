<script lang="ts">
	import { Icons } from '$lib/components/ui/icons';
	import { Button } from '$lib/components/ui/button/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import PlaylistActionItems from './playlist-action-items.svelte';

	let { id, onDeleted }: { id: string; onDeleted?: () => void } = $props();
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger>
		{#snippet child({ props })}
			<Button {...props} variant="ghost" size="icon" class="relative size-8 p-0">
				<span class="sr-only">Open menu</span>
				<Icons.More />
			</Button>
		{/snippet}
	</DropdownMenu.Trigger>
	<DropdownMenu.Content>
		<PlaylistActionItems {id} {onDeleted} />
	</DropdownMenu.Content>
</DropdownMenu.Root>
