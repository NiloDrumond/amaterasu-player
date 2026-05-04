<script lang="ts">
	import { onMount } from 'svelte';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Badge } from '$lib/components/ui/badge';
	import XIcon from '@lucide/svelte/icons/x';
	import PlusIcon from '@lucide/svelte/icons/plus';
	import { toast } from 'svelte-sonner';
	import {
		getTags,
		createTag,
		getTrackTags,
		attachTrackTag,
		detachTrackTag,
		getAlbumTags,
		attachAlbumTag,
		detachAlbumTag,
	} from '$lib/services/tag-service';
	import type { TagResponse } from '$lib/bindings/response/tag/tag-response';
	import type { TagSummaryResponse } from '$lib/bindings/response/tag/tag-summary-response';
	import { resolveTagColor, tagForegroundColor } from './tag-color';

	let {
		entity,
		entityId,
	}: {
		entity: 'track' | 'album';
		entityId: string;
	} = $props();

	let allTags = $state<TagResponse[]>([]);
	let attached = $state<TagSummaryResponse[]>([]);
	let query = $state('');
	let loading = $state(true);

	function listFn() {
		return entity === 'track' ? getTrackTags : getAlbumTags;
	}
	function attachFn() {
		return entity === 'track' ? attachTrackTag : attachAlbumTag;
	}
	function detachFn() {
		return entity === 'track' ? detachTrackTag : detachAlbumTag;
	}

	async function refresh() {
		const [allRes, attachedRes] = await Promise.all([getTags(fetch), listFn()(fetch, entityId)]);
		if (allRes.data) allTags = allRes.data;
		if (attachedRes.data) attached = attachedRes.data;
		loading = false;
	}

	onMount(refresh);

	const attachedIds = $derived(new Set(attached.map((t) => t.id)));

	const suggestions = $derived(
		query.trim()
			? allTags
					.filter(
						(t) =>
							!attachedIds.has(t.id) && t.name.toLowerCase().includes(query.trim().toLowerCase()),
					)
					.slice(0, 8)
			: [],
	);

	const exactMatch = $derived(
		allTags.find((t) => t.name.toLowerCase() === query.trim().toLowerCase()),
	);

	async function attachTag(tagId: string) {
		const { error } = await attachFn()(fetch, entityId, tagId);
		if (error) {
			toast.error(error);
			return;
		}
		query = '';
		await refresh();
	}

	async function detachTag(tagId: string) {
		const { error } = await detachFn()(fetch, entityId, tagId);
		if (error) {
			toast.error(error);
			return;
		}
		await refresh();
	}

	async function createAndAttach() {
		const name = query.trim();
		if (!name) return;
		const { data, error } = await createTag(fetch, {
			name,
			categoryId: null,
			color: null,
		});
		if (error || !data) {
			toast.error(error ?? 'Failed to create tag');
			return;
		}
		await attachTag(data.id);
	}
</script>

<div class="flex flex-col gap-2">
	<div class="flex flex-wrap items-center gap-2">
		{#each attached as tag (tag.id)}
			<Badge
				variant="secondary"
				class="gap-1 pr-1"
				style={tag.color
					? `background-color: ${resolveTagColor(tag.color)}; color: ${tagForegroundColor(tag.color)};`
					: undefined}
			>
				<span>{tag.name}</span>
				<button
					type="button"
					class="rounded-sm opacity-70 hover:opacity-100"
					aria-label="Remove tag {tag.name}"
					onclick={() => detachTag(tag.id)}
				>
					<XIcon class="size-3" />
				</button>
			</Badge>
		{/each}
		{#if !loading && attached.length === 0}
			<span class="text-sm text-muted-foreground">No tags yet</span>
		{/if}
	</div>

	<div class="relative">
		<Input
			bind:value={query}
			placeholder="Add a tag…"
			autocomplete="off"
			onkeydown={(e: KeyboardEvent) => {
				if (e.key === 'Enter') {
					e.preventDefault();
					if (exactMatch && !attachedIds.has(exactMatch.id)) {
						attachTag(exactMatch.id);
					} else if (query.trim() && !exactMatch) {
						createAndAttach();
					}
				}
			}}
		/>
		{#if query.trim() && (suggestions.length > 0 || !exactMatch)}
			<div
				class="absolute top-full right-0 left-0 z-10 mt-1 max-h-60 overflow-auto rounded-md border bg-popover p-1 shadow-md"
			>
				{#each suggestions as tag (tag.id)}
					<button
						type="button"
						class="flex w-full items-center gap-2 rounded-sm px-2 py-1.5 text-left text-sm hover:bg-accent"
						onclick={() => attachTag(tag.id)}
					>
						{#if tag.color}
							<span
								class="size-3 rounded-full border border-border"
								style:background-color={resolveTagColor(tag.color)}
							></span>
						{/if}
						<span>{tag.name}</span>
						{#if tag.category}
							<span class="ml-auto text-xs text-muted-foreground">{tag.category.name}</span>
						{/if}
					</button>
				{/each}
				{#if !exactMatch && query.trim()}
					<button
						type="button"
						class="flex w-full items-center gap-2 rounded-sm px-2 py-1.5 text-left text-sm hover:bg-accent"
						onclick={createAndAttach}
					>
						<PlusIcon class="size-3" />
						<span>Create "{query.trim()}"</span>
					</button>
				{/if}
			</div>
		{/if}
	</div>
</div>
