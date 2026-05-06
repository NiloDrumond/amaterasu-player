<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu/index.js';
	import EllipsisIcon from '@lucide/svelte/icons/ellipsis';
	import PlusIcon from '@lucide/svelte/icons/plus';
	import ChevronUpIcon from '@lucide/svelte/icons/chevron-up';
	import ChevronDownIcon from '@lucide/svelte/icons/chevron-down';
	import { toast } from 'svelte-sonner';
	import { createTag } from '$lib/services/tag-service';
	import { deleteTagCategory } from '$lib/services/tag-category-service';
	import type { TagResponse } from '$lib/bindings/response/tag/tag-response';
	import type { TagCategoryResponse } from '$lib/bindings/response/tag-category/tag-category-response';
	import TagChip from './tag-chip.svelte';
	import { resolveTagColor } from './tag-color';

	let {
		category = null,
		tags,
		onTagClick,
		onCategoryEdit,
		onChanged,
		onMoveUp,
		onMoveDown,
		canMoveUp = false,
		canMoveDown = false,
	}: {
		category?: TagCategoryResponse | null;
		tags: TagResponse[];
		onTagClick: (tag: TagResponse) => void;
		onCategoryEdit?: (category: TagCategoryResponse) => void;
		onChanged: () => void;
		onMoveUp?: () => void;
		onMoveDown?: () => void;
		canMoveUp?: boolean;
		canMoveDown?: boolean;
	} = $props();

	let adding = $state(false);
	let newName = $state('');
	let submitting = $state(false);
	let inputEl = $state<HTMLInputElement | null>(null);

	const swatch = $derived(category?.color ? resolveTagColor(category.color) : null);

	function startAdd() {
		adding = true;
		newName = '';
		queueMicrotask(() => inputEl?.focus());
	}

	function cancelAdd() {
		adding = false;
		newName = '';
	}

	async function submitAdd() {
		const name = newName.trim();
		if (!name) {
			cancelAdd();
			return;
		}
		submitting = true;
		try {
			const { error } = await createTag(fetch, {
				name,
				categoryId: category?.id ?? null,
				color: category?.color ?? null,
			});
			if (error) {
				toast.error(error);
				return;
			}
			newName = '';
			onChanged();
			queueMicrotask(() => inputEl?.focus());
		} finally {
			submitting = false;
		}
	}

	async function handleDeleteCategory() {
		if (!category) return;
		const { error } = await deleteTagCategory(fetch, category.id);
		if (error) {
			toast.error(error);
		} else {
			toast.success('Category deleted');
			onChanged();
		}
	}
</script>

<section class="flex flex-col gap-2 rounded-md border bg-card p-4">
	<header class="flex items-center gap-2">
		{#if swatch}
			<span class="size-3 rounded-full border border-border" style:background-color={swatch}></span>
		{/if}
		<h2 class="tracking-wide uppercase">
			{category?.name ?? 'Uncategorized'}
		</h2>
		<span class="text-xs text-muted-foreground">{tags.length}</span>
		<div class="ml-auto flex items-center gap-1">
			<Button variant="ghost" size="sm" class="gap-1 px-2" onclick={startAdd}>
				<PlusIcon class="size-3.5" />
				Add tag
			</Button>
			{#if category && onCategoryEdit}
				<DropdownMenu.Root>
					<DropdownMenu.Trigger>
						{#snippet child({ props })}
							<Button {...props} variant="ghost" size="icon" class="size-8 p-0">
								<span class="sr-only">Category actions</span>
								<EllipsisIcon class="size-4" />
							</Button>
						{/snippet}
					</DropdownMenu.Trigger>
					<DropdownMenu.Content>
						<DropdownMenu.Group>
							<DropdownMenu.Label>Category</DropdownMenu.Label>
							<DropdownMenu.Item onclick={() => onCategoryEdit(category)}>Edit</DropdownMenu.Item>
							{#if onMoveUp}
								<DropdownMenu.Item disabled={!canMoveUp} onclick={onMoveUp}>
									<ChevronUpIcon class="size-4" />
									Move up
								</DropdownMenu.Item>
							{/if}
							{#if onMoveDown}
								<DropdownMenu.Item disabled={!canMoveDown} onclick={onMoveDown}>
									<ChevronDownIcon class="size-4" />
									Move down
								</DropdownMenu.Item>
							{/if}
							<DropdownMenu.Item
								onclick={handleDeleteCategory}
								class="text-destructive focus:text-destructive"
							>
								Delete
							</DropdownMenu.Item>
						</DropdownMenu.Group>
					</DropdownMenu.Content>
				</DropdownMenu.Root>
			{/if}
		</div>
	</header>

	<div class="flex flex-wrap items-center gap-2">
		{#each tags as tag (tag.id)}
			<TagChip
				name={tag.name}
				color={tag.color ?? category?.color ?? null}
				onClick={() => onTagClick(tag)}
			/>
		{/each}
		{#if tags.length === 0 && !adding}
			<span class="text-sm text-muted-foreground">No tags yet</span>
		{/if}
		{#if adding}
			<Input
				bind:ref={inputEl}
				bind:value={newName}
				placeholder="New tag name"
				autocomplete="off"
				class="h-8 w-40"
				disabled={submitting}
				onkeydown={(e: KeyboardEvent) => {
					if (e.key === 'Enter') {
						e.preventDefault();
						submitAdd();
					} else if (e.key === 'Escape') {
						e.preventDefault();
						cancelAdd();
					}
				}}
				onblur={() => {
					if (!newName.trim()) cancelAdd();
				}}
			/>
		{/if}
	</div>
</section>
