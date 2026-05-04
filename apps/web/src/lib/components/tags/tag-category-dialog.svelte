<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { Dialog } from 'bits-ui';
	import { toast } from 'svelte-sonner';
	import { createTagCategory, updateTagCategory } from '$lib/services/tag-category-service';
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
	<Dialog.Portal>
		<Dialog.Overlay class="fixed inset-0 z-50 bg-black/50" />
		<Dialog.Content
			class="fixed top-1/2 left-1/2 z-50 w-full max-w-md -translate-x-1/2 -translate-y-1/2 rounded-lg bg-secondary p-6 shadow-lg"
		>
			<Dialog.Title class="mb-1 text-lg font-semibold">
				{isEdit ? 'Edit Category' : 'New Category'}
			</Dialog.Title>
			<Dialog.Description class="mb-4 text-sm text-muted-foreground">
				Categories group related tags. Each tag belongs to at most one category.
			</Dialog.Description>
			<form onsubmit={handleSubmit} class="flex flex-col gap-4">
				<div class="flex flex-col gap-2">
					<Label for="category-name">Name</Label>
					<Input
						id="category-name"
						bind:value={name}
						placeholder="e.g. Genre, Mood, Instrument"
						autocomplete="off"
						required
					/>
				</div>
				<div class="flex flex-col gap-2">
					<Label>Color</Label>
					<ColorPicker bind:value={color} />
				</div>
				<div class="flex justify-end gap-2">
					<Dialog.Close>
						{#snippet child({ props })}
							<Button {...props} variant="ghost" type="button">Cancel</Button>
						{/snippet}
					</Dialog.Close>
					<Button type="submit" disabled={submitting || !name.trim()}>
						{submitting ? 'Saving…' : isEdit ? 'Save' : 'Create'}
					</Button>
				</div>
			</form>
		</Dialog.Content>
	</Dialog.Portal>
</Dialog.Root>
