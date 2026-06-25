<script lang="ts">
	import TagChip from '$lib/components/tags/tag-chip.svelte';
	import TagPickerDialog from '$lib/components/tags/tag-picker-dialog.svelte';
	import { Icons } from '$lib/components/ui/icons';
	import type { TrackResponse } from '$lib/bindings/response/track/track-response';

	let { track }: { track: TrackResponse } = $props();

	let open = $state(false);
</script>

<TagPickerDialog entity="track" entityId={track.id} bind:open />

<button
	type="button"
	class="flex flex-wrap items-center gap-1 text-left"
	aria-label="Edit tags"
	onclick={() => (open = true)}
>
	{#if track.tags.length > 0}
		{#each track.tags as tag (tag.id)}
			<TagChip name={tag.name} color={tag.color} />
		{/each}
	{:else}
		<span
			class="inline-flex items-center gap-1 text-xs text-muted-foreground hover:text-foreground"
		>
			<Icons.Add class="size-3" />
			Add
		</span>
	{/if}
</button>
