<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Field, FieldGroup, FieldLabel } from '$lib/components/ui/field/index.js';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { toast } from 'svelte-sonner';
	import { createTag, updateTag, deleteTag } from '$lib/services/tag-service';
	import { invalidateTagsCache } from '$lib/state/tags-cache.svelte';
	import type { TagResponse } from '$lib/bindings/response/tag/tag-response';
	import type { TagCategoryResponse } from '$lib/bindings/response/tag-category/tag-category-response';
	import ColorPicker from './color-picker.svelte';

	let {
		open = $bindable(false),
		tag = null,
		categories = [],
		defaultCategoryId = null,
		onSaved,
	}: {
		open?: boolean;
		tag?: TagResponse | null;
		categories?: TagCategoryResponse[];
		defaultCategoryId?: string | null;
		onSaved: () => void;
	} = $props();

	let name = $state('');
	let categoryId = $state<string>('');
	let color = $state('');
	let submitting = $state(false);
	let confirmingDelete = $state(false);

	const isEdit = $derived(tag !== null);
	const trackCount = $derived(Number(tag?.trackCount ?? 0n));
	const albumCount = $derived(Number(tag?.albumCount ?? 0n));
	const inUse = $derived(trackCount > 0 || albumCount > 0);

	$effect(() => {
		if (open) {
			name = tag?.name ?? '';
			categoryId = tag?.categoryId ?? defaultCategoryId ?? '';
			color = tag?.color ?? '';
			confirmingDelete = false;
		}
	});

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		const trimmedName = name.trim();
		if (!trimmedName) return;

		submitting = true;
		try {
			const trimmedColor = color.trim() ? color.trim() : null;
			const targetCategoryId = categoryId || null;

			const { error } = tag
				? await updateTag(fetch, tag.id, {
						name: trimmedName,
						categoryId: targetCategoryId,
						clearCategory: targetCategoryId === null,
						color: trimmedColor,
					})
				: await createTag(fetch, {
						name: trimmedName,
						categoryId: targetCategoryId,
						color: trimmedColor,
					});

			if (error) {
				toast.error(error);
				return;
			}

			invalidateTagsCache();
			toast.success(isEdit ? 'Tag updated' : 'Tag created');
			open = false;
			onSaved();
		} finally {
			submitting = false;
		}
	}

	function requestDelete() {
		if (inUse) {
			confirmingDelete = true;
		} else {
			handleDelete();
		}
	}

	async function handleDelete() {
		if (!tag) return;
		submitting = true;
		try {
			const { error } = await deleteTag(fetch, tag.id);
			if (error) {
				toast.error(error);
				return;
			}
			invalidateTagsCache();
			toast.success('Tag deleted');
			open = false;
			onSaved();
		} finally {
			submitting = false;
		}
	}
</script>

<Dialog.Root
	bind:open
	onOpenChange={(o) => {
		if (!o) {
			name = '';
			categoryId = '';
			color = '';
			confirmingDelete = false;
		}
	}}
>
	<Dialog.Content>
		{#if confirmingDelete && tag}
			<Dialog.Header>
				<Dialog.Title>Delete tag</Dialog.Title>
				<Dialog.Description>
					Delete "{tag.name}"? It's on
					{#if trackCount > 0}{trackCount}
						{trackCount === 1 ? 'track' : 'tracks'}{/if}{#if trackCount > 0 && albumCount > 0}
						and
					{/if}{#if albumCount > 0}{albumCount}
						{albumCount === 1 ? 'album' : 'albums'}{/if}; this removes the tag from them.
				</Dialog.Description>
			</Dialog.Header>
			<Dialog.Footer>
				<Button variant="ghost" type="button" onclick={() => (confirmingDelete = false)}>
					Cancel
				</Button>
				<Button variant="destructive" onclick={handleDelete} disabled={submitting}>
					{submitting ? 'Deleting…' : 'Delete'}
				</Button>
			</Dialog.Footer>
		{:else}
			<Dialog.Header>
				<Dialog.Title>{isEdit ? 'Edit Tag' : 'New Tag'}</Dialog.Title>
				<Dialog.Description>
					{isEdit ? 'Update this tag.' : 'Tags are personal — only you see and assign them.'}
				</Dialog.Description>
			</Dialog.Header>
			<form onsubmit={handleSubmit}>
				<FieldGroup>
					<Field>
						<FieldLabel for="tag-name">Name</FieldLabel>
						<Input
							id="tag-name"
							bind:value={name}
							placeholder="e.g. piano"
							autocomplete="off"
							required
						/>
					</Field>
					<Field>
						<FieldLabel for="tag-category">Category</FieldLabel>
						<select
							id="tag-category"
							bind:value={categoryId}
							class="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm shadow-xs transition-colors focus-visible:ring-1 focus-visible:ring-ring focus-visible:outline-none"
						>
							<option value="">Uncategorized</option>
							{#each categories as cat (cat.id)}
								<option value={cat.id}>{cat.name}</option>
							{/each}
						</select>
					</Field>
					<Field>
						<FieldLabel>Color</FieldLabel>
						<ColorPicker bind:value={color} />
					</Field>
					<Dialog.Footer>
						{#if isEdit}
							<Button
								variant="ghost"
								type="button"
								class="mr-auto text-destructive hover:text-destructive"
								disabled={submitting}
								onclick={requestDelete}
							>
								Delete
							</Button>
						{/if}
						<Dialog.Close>
							{#snippet child({ props })}
								<Button {...props} variant="ghost" type="button">Cancel</Button>
							{/snippet}
						</Dialog.Close>
						<Button type="submit" disabled={submitting || !name.trim()}>
							{submitting ? 'Saving…' : isEdit ? 'Save' : 'Create'}
						</Button>
					</Dialog.Footer>
				</FieldGroup>
			</form>
		{/if}
	</Dialog.Content>
</Dialog.Root>
