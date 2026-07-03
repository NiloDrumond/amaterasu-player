<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Field, FieldGroup, FieldLabel } from '$lib/components/ui/field/index.js';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import { toast } from 'svelte-sonner';
	import { createTagCategory, updateTagCategory } from '$lib/services/tag-category-service';
	import { invalidateTagsCache } from '$lib/state/tags-cache.svelte';
	import type { TagCategoryResponse } from '$lib/bindings/response/tag-category/tag-category-response';
	import ColorPicker from './color-picker.svelte';

	let {
		open = $bindable(false),
		category = null,
		onSaved,
	}: {
		open?: boolean;
		category?: TagCategoryResponse | null;
		onSaved: () => void;
	} = $props();

	let name = $state('');
	let color = $state('');
	let submitting = $state(false);

	const isEdit = $derived(category !== null);

	$effect(() => {
		if (open) {
			name = category?.name ?? '';
			color = category?.color ?? '';
		}
	});

	async function handleSubmit(e: SubmitEvent) {
		e.preventDefault();
		const trimmedName = name.trim();
		if (!trimmedName) return;

		submitting = true;
		try {
			const params = {
				name: trimmedName,
				color: color.trim() ? color.trim() : null,
			};

			const { error } = category
				? await updateTagCategory(fetch, category.id, params)
				: await createTagCategory(fetch, params);

			if (error) {
				toast.error(error);
				return;
			}

			invalidateTagsCache();
			toast.success(isEdit ? 'Category updated' : 'Category created');
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
			color = '';
		}
	}}
>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>{isEdit ? 'Edit Category' : 'New Category'}</Dialog.Title>
			<Dialog.Description>
				Categories group related tags. Each tag belongs to at most one category.
			</Dialog.Description>
		</Dialog.Header>
		<form onsubmit={handleSubmit}>
			<FieldGroup>
				<Field>
					<FieldLabel for="category-name">Name</FieldLabel>
					<Input
						id="category-name"
						bind:value={name}
						placeholder="e.g. Genre, Mood, Instrument"
						autocomplete="off"
						required
					/>
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
