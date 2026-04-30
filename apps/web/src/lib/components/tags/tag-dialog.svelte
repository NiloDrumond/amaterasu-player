<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Label } from '$lib/components/ui/label/index.js';
	import { Dialog } from 'bits-ui';
	import { toast } from 'svelte-sonner';
	import { createTag, updateTag } from '$lib/services/tag-service';
	import type { TagResponse } from '$lib/bindings/response/tag/tag-response';
	import ColorPicker from './color-picker.svelte';

	let {
		open = $bindable(false),
		tag = null,
		onSaved,
	}: {
		open?: boolean;
		tag?: TagResponse | null;
		onSaved: () => void;
	} = $props();

	let name = $state('');
	let category = $state('');
	let color = $state('');
	let submitting = $state(false);

	const isEdit = $derived(tag !== null);

	$effect(() => {
		if (open) {
			name = tag?.name ?? '';
			category = tag?.category ?? '';
			color = tag?.color ?? '';
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
				category: category.trim() ? category.trim() : null,
				color: color.trim() ? color.trim() : null,
			};

			const { error } = isEdit
				? await updateTag(fetch, tag!.id, params)
				: await createTag(fetch, params);

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
			category = '';
			color = '';
		}
	}}
>
	<Dialog.Portal>
		<Dialog.Overlay class="fixed inset-0 z-50 bg-black/50" />
		<Dialog.Content
			class="fixed top-1/2 left-1/2 z-50 w-full max-w-md -translate-x-1/2 -translate-y-1/2 rounded-lg bg-background p-6 shadow-lg"
		>
			<Dialog.Title class="mb-1 text-lg font-semibold">
				{isEdit ? 'Edit Tag' : 'New Tag'}
			</Dialog.Title>
			<Dialog.Description class="mb-4 text-sm text-muted-foreground">
				{isEdit ? 'Update this tag.' : 'Tags are personal — only you see and assign them.'}
			</Dialog.Description>
			<form onsubmit={handleSubmit} class="flex flex-col gap-4">
				<div class="flex flex-col gap-2">
					<Label for="tag-name">Name</Label>
					<Input
						id="tag-name"
						bind:value={name}
						placeholder="e.g. piano"
						autocomplete="off"
						required
					/>
				</div>
				<div class="flex flex-col gap-2">
					<Label for="tag-category">Category</Label>
					<Input
						id="tag-category"
						bind:value={category}
						placeholder="e.g. genre, vibe, instrument"
						autocomplete="off"
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
