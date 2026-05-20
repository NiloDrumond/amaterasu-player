<script lang="ts">
	import ArtistEditForm from '$lib/components/admin/artist-edit-form.svelte';
	import DataTable from '$lib/components/ui/data-table/data-table.svelte';
	import { albumsColumns } from '$lib/components/albums/columns.js';
	import { tracksColumns } from '$lib/components/tracks/columns.js';
	import { goto, invalidateAll } from '$app/navigation';

	let { data } = $props();
</script>

<div class="space-y-6 p-6">
	<div class="mx-auto max-w-2xl min-w-md space-y-6">
		<header class="flex items-baseline justify-between gap-2">
			<div>
				<h1>Edit artist</h1>
				<p class="text-xs text-muted-foreground">{data.artist.id}</p>
			</div>
			<div class="flex items-center gap-1">
				{#if !data.artist.approved}
					<span
						class="rounded-md bg-amber-500/10 px-2 py-1 text-xs font-medium text-amber-600 dark:text-amber-400"
					>
						Pending
					</span>
				{/if}
				{#if data.artist.lockedAt}
					<span
						class="rounded-md bg-secondary px-2 py-1 text-xs font-medium text-secondary-foreground"
					>
						Locked
					</span>
				{/if}
			</div>
		</header>

		<ArtistEditForm
			artist={data.artist}
			canDelete={data.albums.length === 0}
			onAfterChange={() => invalidateAll()}
			onAfterDelete={() => goto('/admin')}
		/>
	</div>

	<section class="space-y-2">
		<h2>Albums ({data.albums.length})</h2>
		{#if data.albums.length === 0}
			<p class="text-sm text-muted-foreground">No albums. You can hard-delete this artist.</p>
		{:else}
			<DataTable
				storageKey="admin:artist:albums"
				data={data.albums}
				columns={albumsColumns.filter((col) => col.id !== 'artist')}
				onRowClick={(row) => goto(`/admin/albums/${row.id}`)}
			/>
		{/if}
	</section>

	<section class="space-y-2">
		<h2>Tracks ({data.tracks.length})</h2>
		{#if data.tracks.length === 0}
			<p class="text-sm text-muted-foreground">No tracks.</p>
		{:else}
			<DataTable
				storageKey="admin:artist:tracks"
				data={data.tracks}
				columns={tracksColumns}
				onRowClick={(row) => goto(`/admin/tracks/${row.id}`)}
			/>
		{/if}
	</section>
</div>
