<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import { Button } from '$lib/components/ui/button';
	import { Icons } from '$lib/components/ui/icons';
	import TagDialog from '$lib/components/tags/tag-dialog.svelte';
	import TagCategoryDialog from '$lib/components/tags/tag-category-dialog.svelte';
	import TagCategorySection from '$lib/components/tags/tag-category-section.svelte';
	import { reorderTagCategories } from '$lib/services/tag-category-service';
	import { toast } from 'svelte-sonner';
	import type { TagResponse } from '$lib/bindings/response/tag/tag-response';
	import type { TagCategoryResponse } from '$lib/bindings/response/tag-category/tag-category-response';

	let { data } = $props();

	let tagDialogOpen = $state(false);
	let editingTag = $state<TagResponse | null>(null);
	let defaultCategoryId = $state<string | null>(null);

	let categoryDialogOpen = $state(false);
	let editingCategory = $state<TagCategoryResponse | null>(null);

	const categories = $derived(data.categories ?? []);
	const tags = $derived(data.tags ?? []);

	const tagsByCategory = $derived.by(() => {
		const grouped: Record<string, TagResponse[]> = {};
		const uncategorized: TagResponse[] = [];
		for (const cat of categories) grouped[cat.id] = [];
		for (const tag of tags) {
			if (tag.categoryId && grouped[tag.categoryId]) {
				grouped[tag.categoryId].push(tag);
			} else {
				uncategorized.push(tag);
			}
		}
		return { grouped, uncategorized };
	});

	function openCreateTag() {
		editingTag = null;
		defaultCategoryId = null;
		tagDialogOpen = true;
	}

	function openEditTag(tag: TagResponse) {
		editingTag = tag;
		defaultCategoryId = null;
		tagDialogOpen = true;
	}

	function openCreateCategory() {
		editingCategory = null;
		categoryDialogOpen = true;
	}

	function openEditCategory(category: TagCategoryResponse) {
		editingCategory = category;
		categoryDialogOpen = true;
	}

	async function moveCategory(index: number, direction: -1 | 1) {
		const next = [...categories];
		const target = index + direction;
		if (target < 0 || target >= next.length) return;
		[next[index], next[target]] = [next[target], next[index]];
		const { error } = await reorderTagCategories(fetch, {
			orderedIds: next.map((c) => c.id),
		});
		if (error) {
			toast.error(error);
			return;
		}
		await invalidateAll();
	}
</script>

{#if data.error}
	<h1>Error</h1>
	<p>{data.error.message}</p>
{:else}
	<div class="flex flex-col gap-4 p-4">
		<div class="flex items-center justify-between">
			<h1 class="tracking-widest uppercase">Tags</h1>
			<div class="flex gap-2">
				<Button variant="outline" class="gap-2" onclick={openCreateCategory}>
					<Icons.AddFolder class="size-4" />
					New Category
				</Button>
				<Button class="gap-2" onclick={openCreateTag}>
					<Icons.Add class="size-4" />
					New Tag
				</Button>
			</div>
		</div>

		<div class="flex flex-col gap-3">
			{#each categories as category, i (category.id)}
				<TagCategorySection
					{category}
					tags={tagsByCategory.grouped[category.id] ?? []}
					onTagClick={openEditTag}
					onCategoryEdit={openEditCategory}
					onChanged={() => invalidateAll()}
					onMoveUp={() => moveCategory(i, -1)}
					onMoveDown={() => moveCategory(i, 1)}
					canMoveUp={i > 0}
					canMoveDown={i < categories.length - 1}
				/>
			{/each}
			<TagCategorySection
				category={null}
				tags={tagsByCategory.uncategorized}
				onTagClick={openEditTag}
				onChanged={() => invalidateAll()}
			/>
		</div>
	</div>

	<TagDialog
		bind:open={tagDialogOpen}
		tag={editingTag}
		{categories}
		{defaultCategoryId}
		onSaved={() => invalidateAll()}
	/>

	<TagCategoryDialog
		bind:open={categoryDialogOpen}
		category={editingCategory}
		onSaved={() => invalidateAll()}
	/>
{/if}
