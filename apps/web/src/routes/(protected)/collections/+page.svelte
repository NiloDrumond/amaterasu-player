<script lang="ts">
	import { goto } from '$app/navigation';
	import { Icons } from '$lib/components/ui/icons';
	import SearchInput from '$lib/components/filters/search-input.svelte';

	let { data } = $props();

	let query = $state('');

	const filtered = $derived(
		query
			? (data.collections ?? []).filter((c) => c.name.toLowerCase().includes(query.toLowerCase()))
			: (data.collections ?? []),
	);
</script>

{#if data.error}
	<h1>Error</h1>
	<p>{data.error.message}</p>
{:else}
	<div class="flex flex-col gap-3 p-4">
		<h1 class="tracking-widest uppercase">Album Collections</h1>
		{#if data.collections.length === 0}
			<p class="text-sm text-muted-foreground">
				No collections yet. Filter the Albums page and click <strong>Save as collection</strong> to create
				one.
			</p>
		{:else}
			<div class="flex flex-row flex-wrap items-center gap-2">
				<SearchInput
					value={query}
					onChange={(q) => (query = q)}
					placeholder="Search collections…"
				/>
			</div>
			{#if filtered.length === 0}
				<p class="text-sm text-muted-foreground">No collections match “{query}”.</p>
			{:else}
				<div class="flex flex-col">
					{#each filtered as c (c.id)}
						<button
							class="flex items-center gap-3 border-b px-2 py-2 text-left hover:bg-muted/50"
							onclick={() => goto(`/collections/${c.id}`)}
						>
							<Icons.Collection class="size-4 text-muted-foreground" />
							<span class="flex-1 truncate text-sm font-medium">{c.name}</span>
							<span class="text-xs text-muted-foreground">
								{new Date(c.createdAt).toLocaleDateString()}
							</span>
						</button>
					{/each}
				</div>
			{/if}
		{/if}
	</div>
{/if}
