<script lang="ts">
	import { Badge } from '$lib/components/ui/badge';
	import { Icons } from '$lib/components/ui/icons';
	import { resolveTagColor, tagForegroundColor } from './tag-color';
	import type { Snippet } from 'svelte';

	let {
		name,
		color = null,
		onRemove,
		onClick,
		removeAriaLabel,
		children,
	}: {
		name: string;
		color?: string | null;
		onRemove?: () => void;
		onClick?: () => void;
		removeAriaLabel?: string;
		children?: Snippet;
	} = $props();

	const bg = $derived(color ? resolveTagColor(color) : null);
	const fg = $derived(color ? tagForegroundColor(color) : null);
</script>

{#if onClick}
	<button
		type="button"
		onclick={onClick}
		class="inline-flex items-center rounded-md transition-opacity hover:opacity-80"
	>
		<Badge
			variant="secondary"
			class={onRemove ? 'gap-1 pr-1' : 'gap-1'}
			style={bg ? `background-color: ${bg}; color: ${fg};` : undefined}
		>
			<span>{name}</span>
			{#if children}{@render children()}{/if}
			{#if onRemove}
				<button
					type="button"
					class="rounded-sm opacity-70 hover:opacity-100"
					aria-label={removeAriaLabel ?? `Remove ${name}`}
					onclick={(e) => {
						e.stopPropagation();
						onRemove?.();
					}}
				>
					<Icons.Close class="size-3" />
				</button>
			{/if}
		</Badge>
	</button>
{:else}
	<Badge
		variant="secondary"
		class={onRemove ? 'gap-1 pr-1' : 'gap-1'}
		style={bg ? `background-color: ${bg}; color: ${fg};` : undefined}
	>
		<span>{name}</span>
		{#if children}{@render children()}{/if}
		{#if onRemove}
			<button
				type="button"
				class="rounded-sm opacity-70 hover:opacity-100"
				aria-label={removeAriaLabel ?? `Remove ${name}`}
				onclick={onRemove}
			>
				<Icons.Close class="size-3" />
			</button>
		{/if}
	</Badge>
{/if}
