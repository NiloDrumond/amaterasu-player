<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Field, FieldGroup, FieldLabel } from '$lib/components/ui/field/index.js';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { toast } from 'svelte-sonner';
	import { createTag, updateTag } from '$lib/services/tag-service';
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

	const isEdit = $derived(tag !== null);

	$effect(() => {
		if (open) {
			name = tag?.name ?? '';
			categoryId = tag?.categoryId ?? defaultCategoryId ?? '';
			color = tag?.color ?? '';
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

			toast.success(isEdit ? 'Tag updated' : 'Tag created');
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
		}
	}}
>
	<Dialog.Content>
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
	</Dialog.Content>
</Dialog.Root>
