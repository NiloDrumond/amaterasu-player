<script lang="ts">
	import { untrack } from 'svelte';
	import { Input } from '$lib/components/ui/input/index.js';
	import SearchIcon from '@lucide/svelte/icons/search';
	import XIcon from '@lucide/svelte/icons/x';

	let {
		value,
		onChange,
		placeholder = 'Search…',
		debounceMs = 200,
	}: {
		/** Source of truth from the parent (URL state). */
		value: string;
		/** Called with the trimmed query (or empty string) after debounce. */
		onChange: (query: string) => void;
		placeholder?: string;
		debounceMs?: number;
	} = $props();

	// Local draft for the input — keeps typing snappy without waiting for the parent to round-trip.
	let draft = $state('');
	let lastSyncedValue = $state('');
	let timer: ReturnType<typeof setTimeout> | null = null;

	$effect(() => {
		// Resync ONLY when the upstream `value` itself changes (e.g. back button).
		// `lastSyncedValue` is read untracked so that mutating it inside `commit()`
		// does not re-fire this effect against a stale `value`.
		const next = value;
		untrack(() => {
			if (next !== lastSyncedValue) {
				draft = next;
				lastSyncedValue = next;
			}
		});
	});

	function commit(q: string) {
		const trimmed = q.trim();
		if (trimmed === lastSyncedValue) return;
		lastSyncedValue = trimmed;
		onChange(trimmed);
	}

	function onInput(e: Event) {
		const v = (e.target as HTMLInputElement).value;
		draft = v;
		if (timer) clearTimeout(timer);
		timer = setTimeout(() => commit(v), debounceMs);
	}

	function clear() {
		draft = '';
		if (timer) clearTimeout(timer);
		commit('');
	}
</script>

<div class="relative w-72">
	<SearchIcon
		class="pointer-events-none absolute top-1/2 left-2.5 size-3.5 -translate-y-1/2 text-muted-foreground"
	/>
	<Input
		value={draft}
		oninput={onInput}
		{placeholder}
		class="h-8 pr-8 pl-8 text-xs"
		autocomplete="off"
	/>
	{#if draft}
		<button
			type="button"
			class="absolute top-1/2 right-2 -translate-y-1/2 rounded-sm text-muted-foreground hover:text-foreground"
			aria-label="Clear search"
			onclick={clear}
		>
			<XIcon class="size-3.5" />
		</button>
	{/if}
</div>
