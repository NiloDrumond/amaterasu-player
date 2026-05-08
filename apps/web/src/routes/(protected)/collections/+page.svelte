<script lang="ts">
	import { goto } from '$app/navigation';
	import LayoutGridIcon from '@lucide/svelte/icons/layout-grid';

	let { data } = $props();
</script>

{#if data.error}
	<h1>Error</h1>
	<p>{data.error.message}</p>
{:else}
	<div class="flex flex-col p-4">
		<h1 class="mb-4 tracking-widest uppercase">Album Collections</h1>
		{#if data.collections.length === 0}
			<p class="text-sm text-muted-foreground">
				No collections yet. Filter the Albums page and click <strong>Save as collection</strong> to create
				one.
			</p>
		{:else}
			<div class="flex flex-col">
				{#each data.collections as c (c.id)}
					<button
						class="flex items-center gap-3 border-b px-2 py-2 text-left hover:bg-muted/50"
						onclick={() => goto(`/collections/${c.id}`)}
					>
						<LayoutGridIcon class="size-4 text-muted-foreground" />
						<span class="flex-1 truncate text-sm font-medium">{c.name}</span>
						<span class="text-xs text-muted-foreground">
							{new Date(c.createdAt).toLocaleDateString()}
						</span>
					</button>
				{/each}
			</div>
		{/if}
	</div>
{/if}
