<script lang="ts">
	import type { Snippet } from 'svelte';
	import { Button } from '$lib/components/ui/button';
	import { Icons } from '$lib/components/ui/icons';

	let {
		title,
		seeAllHref,
		emptyMessage,
		isEmpty = false,
		children,
	}: {
		title: string;
		seeAllHref?: string;
		emptyMessage?: string;
		isEmpty?: boolean;
		children: Snippet;
	} = $props();

	let scroller: HTMLDivElement | undefined = $state();

	function scroll(direction: -1 | 1) {
		if (!scroller) return;
		scroller.scrollBy({ left: direction * scroller.clientWidth * 0.8, behavior: 'smooth' });
	}
</script>

<section class="group/section">
	<header class="mb-2 flex items-baseline justify-between">
		<h2 class="text-sm font-semibold tracking-wide text-muted-foreground uppercase">{title}</h2>
		<div class="flex items-center gap-1">
			{#if seeAllHref}
				<a href={seeAllHref} class="text-xs text-muted-foreground hover:text-foreground">
					See all
				</a>
			{/if}
			{#if !isEmpty}
				<Button
					variant="ghost"
					size="icon"
					class="size-7 opacity-0 transition-opacity group-hover/section:opacity-100"
					aria-label="Scroll left"
					onclick={() => scroll(-1)}
				>
					<Icons.GoLeft />
				</Button>
				<Button
					variant="ghost"
					size="icon"
					class="size-7 opacity-0 transition-opacity group-hover/section:opacity-100"
					aria-label="Scroll right"
					onclick={() => scroll(1)}
				>
					<Icons.GoRight />
				</Button>
			{/if}
		</div>
	</header>

	{#if isEmpty && emptyMessage}
		<p class="text-sm text-muted-foreground">{emptyMessage}</p>
	{:else}
		<div bind:this={scroller} class="scrollbar-none flex gap-3 overflow-x-auto scroll-smooth">
			{@render children()}
		</div>
	{/if}
</section>
