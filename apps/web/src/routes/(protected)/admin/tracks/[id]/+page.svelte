<script lang="ts">
	import TrackEditForm from '$lib/components/admin/track-edit-form.svelte';
	import { invalidateAll } from '$app/navigation';

	let { data } = $props();
</script>

<div class="mx-auto max-w-2xl min-w-md space-y-6 p-6">
	<header class="flex items-baseline justify-between gap-2">
		<div>
			<h1>Edit track</h1>
			<p class="text-xs text-muted-foreground">{data.track.id}</p>
		</div>
		<div class="flex items-center gap-1">
			{#if !data.track.approved}
				<span
					class="rounded-md bg-amber-500/10 px-2 py-1 text-xs font-medium text-amber-600 dark:text-amber-400"
				>
					Pending
				</span>
			{/if}
			{#if data.track.lockedAt}
				<span
					class="rounded-md bg-secondary px-2 py-1 text-xs font-medium text-secondary-foreground"
					title="Locked at {data.track.lockedAt}"
				>
					Locked
				</span>
			{/if}
			{#if data.track.deletedAt}
				<span class="rounded-md bg-red-500/10 px-2 py-1 text-xs font-medium text-red-500">
					Soft-deleted
				</span>
			{/if}
		</div>
	</header>

	<TrackEditForm
		track={data.track}
		artist={data.artist}
		album={data.album}
		onAfterChange={() => invalidateAll()}
	/>
</div>
