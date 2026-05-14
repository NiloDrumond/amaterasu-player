<script lang="ts">
	import type { Entity, LeafNode } from './filter-types';
	import { findFieldSpec } from './filter-types';
	import { Icons } from '$lib/components/ui/icons';
	import { Badge } from '$lib/components/ui/badge';

	let {
		entity,
		leaf,
		labelOverride,
		onRemove,
		onToggleNegate,
	}: {
		entity: Entity;
		leaf: LeafNode;
		/** Optional human-friendly value label (e.g. tag name, album title) */
		labelOverride?: string | null;
		onRemove: () => void;
		onToggleNegate: () => void;
	} = $props();

	const spec = $derived(findFieldSpec(entity, leaf.field));
	const valueLabel = $derived.by(() => {
		if (labelOverride) return labelOverride;
		const v = leaf.value;
		switch (v.kind) {
			case 'text':
				return `"${v.value}"`;
			case 'id':
				return v.value.slice(0, 8);
			case 'ids':
				return `${v.value.length} items`;
			case 'intRange':
				if (v.min !== null && v.max !== null) return `${v.min}–${v.max}`;
				if (v.min !== null) return `≥ ${v.min}`;
				if (v.max !== null) return `≤ ${v.max}`;
				return 'any';
		}
	});

	const operatorLabel = $derived.by(() => {
		if (leaf.op === 'contains') return 'contains';
		if (leaf.op === 'between') return 'in';
		return 'is';
	});
</script>

<Badge variant="outline" class="gap-1.5 pr-1 font-normal">
	<span class="text-muted-foreground">{spec?.label ?? leaf.field}</span>
	<button
		type="button"
		class="text-foreground hover:underline"
		title={leaf.negate ? 'Click to remove negation' : 'Click to negate'}
		onclick={onToggleNegate}
	>
		{leaf.negate ? `is not` : operatorLabel}
	</button>
	<span class="font-medium">{valueLabel}</span>
	<button
		type="button"
		class="rounded-sm opacity-70 hover:opacity-100"
		aria-label="Remove filter"
		onclick={onRemove}
	>
		<Icons.Close class="size-3" />
	</button>
</Badge>
