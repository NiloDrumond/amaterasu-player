<script lang="ts">
	import type { Snippet } from 'svelte';
	import { Icons } from '$lib/components/ui/icons';

	let {
		coverUrl,
		title,
		subtitle,
		href,
		onclick,
		fallbackIcon = 'music',
		overlay,
	}: {
		coverUrl?: string | null;
		title: string;
		subtitle?: string | null;
		href?: string;
		onclick?: () => void;
		fallbackIcon?: 'music' | 'playlist';
		overlay?: Snippet;
	} = $props();

	const Fallback = $derived(fallbackIcon === 'playlist' ? Icons.Playlist : Icons.Music);
</script>

{#snippet inner()}
	<div class="relative aspect-square w-full overflow-hidden bg-muted">
		{#if coverUrl}
			<img src={coverUrl} alt="" loading="lazy" class="size-full object-cover" />
		{:else}
			<div class="flex size-full items-center justify-center text-muted-foreground">
				<Fallback class="size-10" />
			</div>
		{/if}
		{#if overlay}
			<div class="absolute top-1 right-1 opacity-0 transition-opacity group-hover/card:opacity-100">
				{@render overlay()}
			</div>
		{/if}
	</div>
	<div class="px-1 pt-2">
		<p class="truncate text-sm font-medium" {title}>{title}</p>
		{#if subtitle}
			<p class="truncate text-xs text-muted-foreground" title={subtitle}>{subtitle}</p>
		{/if}
	</div>
{/snippet}

{#if href}
	<a {href} class="group/card block w-40 shrink-0 text-foreground hover:text-foreground">
		{@render inner()}
	</a>
{:else}
	<button
		type="button"
		{onclick}
		class="group/card block w-40 shrink-0 text-left text-foreground hover:text-foreground"
	>
		{@render inner()}
	</button>
{/if}
