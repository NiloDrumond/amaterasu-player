<script lang="ts">
	import type { FilterNode } from '$lib/bindings/filter/filter-node';
	import type { Snippet } from 'svelte';
	import { SvelteMap } from 'svelte/reactivity';
	import FilterChip from './filter-chip.svelte';
	import FilterAddPopover from './filter-add-popover.svelte';
	import { type Entity, type LeafNode, isLeaf } from './filter-types';
	import { resolveValueLabel } from './resolve-labels';

	let {
		entity,
		filter,
		onChange,
		leading,
		trailing,
	}: {
		entity: Entity;
		filter: FilterNode | null;
		/**
		 * Called whenever the filter tree changes. `next` is null if no filters
		 * are active.
		 */
		onChange: (next: FilterNode | null) => void;
		/** Rendered on the controls row, before the add-filter button. */
		leading?: Snippet;
		/** Rendered on the controls row, pushed to the right. */
		trailing?: Snippet;
	} = $props();

	/**
	 * Flat-mode wrapper: we only edit a top-level AND group of leaves.
	 * Nested groups are left untouched (advanced mode hook).
	 */
	const allChildren = $derived.by<FilterNode[]>(() => {
		if (!filter) return [];
		return filter.kind === 'group' && filter.op === 'and' && !filter.negate
			? filter.children
			: [filter];
	});

	function isOwnedBySearchInput(c: FilterNode): boolean {
		return c.kind === 'leaf' && c.field === 'text' && !c.negate;
	}

	// Visible chips exclude the text leaf (owned by the always-visible SearchInput).
	const children = $derived(allChildren.filter((c) => !isOwnedBySearchInput(c)));
	// Hidden — preserved on every mutation so the search input's leaf isn't dropped.
	const hidden = $derived(allChildren.filter(isOwnedBySearchInput));

	/**
	 * Human-readable labels keyed by leaf identity (field + value content).
	 * Populated up-front when the user picks a value via the popover, and
	 * resolved lazily for leaves loaded from the URL or a saved playlist
	 * filter (which only carry raw ids).
	 */
	const labels = new SvelteMap<string, string>();

	function leafKey(leaf: LeafNode): string {
		return `${leaf.field}:${JSON.stringify(leaf.value)}`;
	}

	// Resolve display names for id-valued leaves that arrived without a label
	// (e.g. from a persisted filter). `resolveValueLabel` dedupes lookups, so
	// calling it on every re-run for an unresolved leaf is cheap.
	$effect(() => {
		for (const child of children) {
			if (!isLeaf(child) || child.value.kind !== 'id') continue;
			const key = leafKey(child);
			if (labels.has(key)) continue;
			const id = child.value.value;
			if (!id) continue;
			void resolveValueLabel(child.field, id).then((name) => {
				if (name) labels.set(key, name);
			});
		}
	});

	function setChildren(next: FilterNode[]) {
		const merged = [...hidden, ...next];
		if (merged.length === 0) {
			onChange(null);
			return;
		}
		onChange({ kind: 'group', op: 'and', negate: false, children: merged });
	}

	function addLeaf(leaf: LeafNode, label: string) {
		labels.set(leafKey(leaf), label);
		setChildren([...children, leaf]);
	}

	function removeAt(idx: number) {
		setChildren(children.filter((_, i) => i !== idx));
	}

	function toggleNegateAt(idx: number) {
		const next = children.map((c, i) => {
			if (i !== idx) return c;
			return { ...c, negate: !c.negate };
		});
		setChildren(next);
	}

	function clear() {
		setChildren([]);
	}
</script>

<div class="flex flex-col gap-2">
	<div class="flex flex-row flex-wrap items-center gap-2">
		{@render leading?.()}
		<FilterAddPopover {entity} onAdd={addLeaf} />
		{#if children.length > 0}
			<button
				type="button"
				class="text-xs text-muted-foreground hover:text-foreground hover:underline"
				onclick={clear}
			>
				Clear all
			</button>
		{/if}
		<div class="ml-auto"></div>
		{@render trailing?.()}
	</div>
	{#if children.length > 0}
		<div class="flex flex-wrap items-center gap-1.5">
			{#each children as child, idx (isLeaf(child) ? leafKey(child) : idx)}
				{#if isLeaf(child)}
					<FilterChip
						{entity}
						leaf={child}
						labelOverride={labels.get(leafKey(child))}
						onRemove={() => removeAt(idx)}
						onToggleNegate={() => toggleNegateAt(idx)}
					/>
				{/if}
			{/each}
		</div>
	{/if}
</div>
