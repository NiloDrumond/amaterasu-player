<script lang="ts" generics="T extends { id: string }">
	import * as Command from '$lib/components/ui/command/index.js';
	import * as Popover from '$lib/components/ui/popover/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Field, FieldLabel } from '$lib/components/ui/field/index.js';
	import CaretDown from 'phosphor-svelte/lib/CaretDown';
	import { onDestroy } from 'svelte';

	let {
		value = $bindable(),
		label,
		placeholder = 'Search…',
		emptyText = 'No matches',
		formatLabel,
		search,
		onCreate,
		canCreate = true,
	}: {
		value: T | null;
		label: string;
		placeholder?: string;
		emptyText?: string;
		formatLabel: (item: T) => string;
		search: (q: string) => Promise<T[]>;
		onCreate?: (q: string) => Promise<T | null>;
		canCreate?: boolean;
	} = $props();

	let open = $state(false);
	let query = $state('');
	let results = $state<T[]>([]);
	let loading = $state(false);
	let creating = $state(false);
	let timer: ReturnType<typeof setTimeout> | null = null;

	function runSearch(q: string) {
		if (timer) clearTimeout(timer);
		const trimmed = q.trim();
		if (trimmed.length === 0) {
			results = [];
			loading = false;
			return;
		}
		loading = true;
		timer = setTimeout(async () => {
			const data = await search(trimmed);
			results = data;
			loading = false;
		}, 200);
	}

	$effect(() => {
		runSearch(query);
	});

	onDestroy(() => {
		if (timer) clearTimeout(timer);
	});

	const showCreate = $derived(
		canCreate &&
			!!onCreate &&
			query.trim().length > 0 &&
			!results.some((r) => formatLabel(r).toLowerCase() === query.trim().toLowerCase()),
	);

	async function handleCreate() {
		if (!onCreate) return;
		const q = query.trim();
		if (!q) return;
		creating = true;
		try {
			const created = await onCreate(q);
			if (created) {
				value = created;
				open = false;
				query = '';
			}
		} finally {
			creating = false;
		}
	}

	function pick(item: T) {
		value = item;
		open = false;
		query = '';
	}

	function clear() {
		value = null;
	}
</script>

<Field>
	<div class="flex items-center justify-between">
		<FieldLabel>{label}</FieldLabel>
		{#if value}
			<button
				type="button"
				class="text-xs text-muted-foreground underline hover:text-foreground"
				onclick={clear}
			>
				Clear
			</button>
		{/if}
	</div>
	<Popover.Root bind:open>
		<Popover.Trigger>
			{#snippet child({ props })}
				<Button
					variant="outline"
					class="w-full justify-between font-normal"
					role="combobox"
					{...props}
				>
					<span class={value ? '' : 'text-muted-foreground'}>
						{value ? formatLabel(value) : placeholder}
					</span>
					<CaretDown class="size-4 opacity-50" />
				</Button>
			{/snippet}
		</Popover.Trigger>
		<Popover.Content class="w-[--bits-popover-anchor-width] p-0" align="start">
			<Command.Root shouldFilter={false}>
				<Command.Input bind:value={query} {placeholder} />
				<Command.List>
					{#if loading}
						<Command.Loading>Searching…</Command.Loading>
					{:else if results.length === 0 && !showCreate}
						<Command.Empty>{emptyText}</Command.Empty>
					{:else}
						<Command.Group>
							{#each results as item (item.id)}
								<Command.Item value={item.id} onSelect={() => pick(item)}>
									{formatLabel(item)}
								</Command.Item>
							{/each}
						</Command.Group>
						{#if showCreate}
							<Command.Separator />
							<Command.Group>
								<Command.Item value="__create__" disabled={creating} onSelect={handleCreate}>
									+ Create "{query.trim()}"
								</Command.Item>
							</Command.Group>
						{/if}
					{/if}
				</Command.List>
			</Command.Root>
		</Popover.Content>
	</Popover.Root>
</Field>
