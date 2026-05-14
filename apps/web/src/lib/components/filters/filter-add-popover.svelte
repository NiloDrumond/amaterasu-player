<script lang="ts">
	import * as Popover from '$lib/components/ui/popover/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Icons } from '$lib/components/ui/icons';
	import {
		fieldSpecsFor,
		defaultLeafForField,
		type Entity,
		type FieldSpec,
		type LeafNode,
	} from './filter-types';
	import { getTags } from '$lib/services/tag-service';
	import { searchAlbums } from '$lib/services/album-service';
	import { searchArtists } from '$lib/services/artist-service';
	import type { TagResponse } from '$lib/bindings/response/tag/tag-response';

	let {
		entity,
		onAdd,
	}: {
		entity: Entity;
		onAdd: (leaf: LeafNode, label: string) => void;
	} = $props();

	let open = $state(false);
	let stage: 'pick-field' | 'pick-value' = $state('pick-field');
	let chosenSpec: FieldSpec | null = $state(null);
	let draft: LeafNode | null = $state(null);
	let draftLabel = $state('');

	let textValue = $state('');
	let minValue = $state('');
	let maxValue = $state('');
	let searchQuery = $state('');
	let tagOptions = $state<TagResponse[]>([]);
	let albumOptions = $state<{ id: string; title: string }[]>([]);
	let artistOptions = $state<{ id: string; name: string }[]>([]);
	let searchTimer: ReturnType<typeof setTimeout> | null = null;

	const fields = $derived(fieldSpecsFor(entity));

	function reset() {
		stage = 'pick-field';
		chosenSpec = null;
		draft = null;
		draftLabel = '';
		textValue = '';
		minValue = '';
		maxValue = '';
		searchQuery = '';
		tagOptions = [];
		albumOptions = [];
		artistOptions = [];
		if (searchTimer) clearTimeout(searchTimer);
	}

	async function pickField(spec: FieldSpec) {
		chosenSpec = spec;
		draft = defaultLeafForField(spec);
		stage = 'pick-value';
		if (spec.editor === 'tag') {
			const { data } = await getTags(fetch);
			if (data) tagOptions = data;
		}
	}

	function debounceSearch(fn: () => Promise<void>) {
		if (searchTimer) clearTimeout(searchTimer);
		searchTimer = setTimeout(fn, 200);
	}

	$effect(() => {
		if (!chosenSpec) return;
		if (chosenSpec.editor === 'album') {
			const q = searchQuery.trim();
			if (!q) {
				albumOptions = [];
				return;
			}
			debounceSearch(async () => {
				const { data } = await searchAlbums(fetch, q);
				if (data) albumOptions = data.map((a) => ({ id: a.id, title: a.title }));
			});
		} else if (chosenSpec.editor === 'artist') {
			const q = searchQuery.trim();
			if (!q) {
				artistOptions = [];
				return;
			}
			debounceSearch(async () => {
				const { data } = await searchArtists(fetch, q);
				if (data) artistOptions = data.map((a) => ({ id: a.id, name: a.name }));
			});
		}
	});

	function commit() {
		if (!draft || !chosenSpec) return;
		onAdd(draft, draftLabel);
		open = false;
		reset();
	}

	function pickIdValue(id: string, label: string) {
		if (!draft) return;
		draft = { ...draft, value: { kind: 'id', value: id } };
		draftLabel = label;
		commit();
	}

	function commitText() {
		if (!draft || !chosenSpec) return;
		const v = textValue.trim();
		if (!v) return;
		draft = { ...draft, value: { kind: 'text', value: v } };
		draftLabel = `"${v}"`;
		commit();
	}

	function commitRange() {
		if (!draft || !chosenSpec) return;
		const min = minValue.trim() === '' ? null : Number(minValue);
		const max = maxValue.trim() === '' ? null : Number(maxValue);
		if (min === null && max === null) return;
		const minBig = min === null ? null : (BigInt(Math.trunc(min)) as unknown as bigint);
		const maxBig = max === null ? null : (BigInt(Math.trunc(max)) as unknown as bigint);
		draft = { ...draft, value: { kind: 'intRange', min: minBig, max: maxBig } };
		draftLabel =
			min !== null && max !== null ? `${min}–${max}` : min !== null ? `≥ ${min}` : `≤ ${max}`;
		commit();
	}

	function onOpenChange(v: boolean) {
		open = v;
		if (!v) reset();
	}
</script>

<Popover.Root bind:open {onOpenChange}>
	<Popover.Trigger>
		{#snippet child({ props })}
			<Button {...props} variant="outline" size="sm" class="h-7 gap-1 px-2 text-xs">
				<Icons.Add class="size-3" />
				Add filter
			</Button>
		{/snippet}
	</Popover.Trigger>
	<Popover.Content class="w-64 p-1.5" align="start">
		{#if stage === 'pick-field'}
			<div class="flex flex-col gap-0.5">
				<div class="px-2 py-1 text-xs text-muted-foreground">Filter by</div>
				{#each fields as spec (spec.field)}
					<button
						type="button"
						class="rounded-sm px-2 py-1.5 text-left text-xs hover:bg-accent"
						onclick={() => pickField(spec)}
					>
						{spec.label}
					</button>
				{/each}
			</div>
		{:else if chosenSpec}
			<div class="flex flex-col gap-2 p-1">
				<div class="text-xs text-muted-foreground">{chosenSpec.label}</div>
				{#if chosenSpec.editor === 'text'}
					<Input
						bind:value={textValue}
						placeholder="Search…"
						autofocus
						onkeydown={(e: KeyboardEvent) => {
							if (e.key === 'Enter') {
								e.preventDefault();
								commitText();
							}
						}}
					/>
					<Button size="sm" onclick={commitText} disabled={!textValue.trim()}>Add</Button>
				{:else if chosenSpec.editor === 'tag'}
					<Input bind:value={searchQuery} placeholder="Search tags…" autofocus />
					<div class="max-h-48 flex-col overflow-auto">
						{#each tagOptions.filter((t) => searchQuery.trim() === '' || t.name
									.toLowerCase()
									.includes(searchQuery.trim().toLowerCase())) as t (t.id)}
							<button
								type="button"
								class="block w-full rounded-sm px-2 py-1 text-left text-xs hover:bg-accent"
								onclick={() => pickIdValue(t.id, t.name)}
							>
								{t.name}
							</button>
						{:else}
							<div class="px-2 py-1 text-xs text-muted-foreground">No tags</div>
						{/each}
					</div>
				{:else if chosenSpec.editor === 'album'}
					<Input bind:value={searchQuery} placeholder="Search albums…" autofocus />
					<div class="max-h-48 flex-col overflow-auto">
						{#each albumOptions as a (a.id)}
							<button
								type="button"
								class="block w-full rounded-sm px-2 py-1 text-left text-xs hover:bg-accent"
								onclick={() => pickIdValue(a.id, a.title)}
							>
								{a.title}
							</button>
						{:else}
							<div class="px-2 py-1 text-xs text-muted-foreground">
								{searchQuery.trim() ? 'No matches' : 'Type to search'}
							</div>
						{/each}
					</div>
				{:else if chosenSpec.editor === 'artist'}
					<Input bind:value={searchQuery} placeholder="Search artists…" autofocus />
					<div class="max-h-48 flex-col overflow-auto">
						{#each artistOptions as a (a.id)}
							<button
								type="button"
								class="block w-full rounded-sm px-2 py-1 text-left text-xs hover:bg-accent"
								onclick={() => pickIdValue(a.id, a.name)}
							>
								{a.name}
							</button>
						{:else}
							<div class="px-2 py-1 text-xs text-muted-foreground">
								{searchQuery.trim() ? 'No matches' : 'Type to search'}
							</div>
						{/each}
					</div>
				{:else if chosenSpec.editor === 'yearRange' || chosenSpec.editor === 'intRange'}
					<div class="flex gap-1">
						<Input bind:value={minValue} placeholder="Min" type="number" />
						<Input bind:value={maxValue} placeholder="Max" type="number" />
					</div>
					{#if chosenSpec.editor === 'intRange'}
						<div class="text-[10px] text-muted-foreground">in milliseconds</div>
					{/if}
					<Button size="sm" onclick={commitRange}>Add</Button>
				{/if}
			</div>
		{/if}
	</Popover.Content>
</Popover.Root>
