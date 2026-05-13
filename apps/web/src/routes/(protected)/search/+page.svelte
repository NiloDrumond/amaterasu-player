<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/state';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input/index.js';
	import { searchAll, type SearchEntityType, type SearchHit } from '$lib/services/search-service';
	import { onMount } from 'svelte';

	const TYPES: Array<{ value: SearchEntityType; label: string }> = [
		{ value: 'track', label: 'Tracks' },
		{ value: 'album', label: 'Albums' },
		{ value: 'artist', label: 'Artists' },
		{ value: 'playlist', label: 'Playlists' },
		{ value: 'collection', label: 'Collections' },
	];

	const PAGE_SIZE = 25;

	let q = $derived(page.url.searchParams.get('q') ?? '');
	let type = $derived((page.url.searchParams.get('type') as SearchEntityType | null) ?? 'track');
	let pageNum = $derived(Number.parseInt(page.url.searchParams.get('page') ?? '1', 10));

	let queryInput = $state('');
	let hits = $state<SearchHit[]>([]);
	let loading = $state(false);
	let lastFetchKey = '';

	onMount(() => {
		queryInput = q;
	});

	$effect(() => {
		const key = `${q}::${type}::${pageNum}`;
		if (key === lastFetchKey) return;
		lastFetchKey = key;
		if (!q) {
			hits = [];
			return;
		}
		loading = true;
		searchAll(fetch, { q, type, page: pageNum, pageSize: PAGE_SIZE }).then((res) => {
			hits = res.data ?? [];
			loading = false;
		});
	});

	function setUrl(params: { q?: string; type?: SearchEntityType; page?: number }) {
		const url = new URL(page.url);
		if (params.q !== undefined) {
			if (params.q) url.searchParams.set('q', params.q);
			else url.searchParams.delete('q');
		}
		if (params.type !== undefined) url.searchParams.set('type', params.type);
		if (params.page !== undefined) {
			if (params.page > 1) url.searchParams.set('page', String(params.page));
			else url.searchParams.delete('page');
		}
		goto(url, { keepFocus: true, noScroll: true });
	}

	function onSubmit(e: SubmitEvent) {
		e.preventDefault();
		setUrl({ q: queryInput.trim(), page: 1 });
	}

	function selectType(t: SearchEntityType) {
		setUrl({ type: t, page: 1 });
	}

	function hrefFor(hit: SearchHit): string {
		switch (hit.kind) {
			case 'album':
				return `/albums/${hit.id}`;
			case 'artist':
				return `/artists/${hit.id}`;
			case 'playlist':
				return `/playlists/${hit.id}`;
			case 'collection':
				return `/collections/${hit.id}`;
			case 'track':
				// Tracks open the album page (full track detail not yet a route).
				return `/tracks?q=${encodeURIComponent(hit.title)}`;
		}
	}
</script>

<div class="flex flex-col gap-4 p-6">
	<form onsubmit={onSubmit} class="flex max-w-xl gap-2">
		<Input bind:value={queryInput} placeholder="Search the library…" />
		<Button type="submit">Search</Button>
	</form>

	<div class="flex gap-1 border-b">
		{#each TYPES as t (t.value)}
			<button
				type="button"
				class="-mb-px border-b-2 px-3 py-1.5 text-sm transition-colors"
				class:border-foreground={type === t.value}
				class:border-transparent={type !== t.value}
				class:text-muted-foreground={type !== t.value}
				onclick={() => selectType(t.value)}
			>
				{t.label}
			</button>
		{/each}
	</div>

	{#if !q}
		<p class="text-sm text-muted-foreground">Enter a query above to search the library.</p>
	{:else if loading}
		<p class="text-sm text-muted-foreground">Searching…</p>
	{:else if hits.length === 0}
		<p class="text-sm text-muted-foreground">No results for "{q}".</p>
	{:else}
		<ul class="flex flex-col divide-y rounded border">
			{#each hits as hit (hit.id)}
				<li>
					<a href={hrefFor(hit)} class="flex flex-col px-3 py-2 hover:bg-muted">
						<span class="font-medium">{hit.title}</span>
						{#if hit.subtitle}
							<span class="text-xs text-muted-foreground">{hit.subtitle}</span>
						{/if}
					</a>
				</li>
			{/each}
		</ul>

		<div class="flex items-center gap-2">
			<Button
				variant="outline"
				size="sm"
				disabled={pageNum <= 1}
				onclick={() => setUrl({ page: pageNum - 1 })}
			>
				Previous
			</Button>
			<span class="text-xs text-muted-foreground">Page {pageNum}</span>
			<Button
				variant="outline"
				size="sm"
				disabled={hits.length < PAGE_SIZE}
				onclick={() => setUrl({ page: pageNum + 1 })}
			>
				Next
			</Button>
		</div>
	{/if}
</div>
