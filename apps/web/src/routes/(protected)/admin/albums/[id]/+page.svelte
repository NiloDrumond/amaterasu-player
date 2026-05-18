<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import EntityPicker from '$lib/components/admin/entity-picker.svelte';
	import AlbumEditForm from '$lib/components/admin/album-edit-form.svelte';
	import DataTable from '$lib/components/ui/data-table/data-table.svelte';
	import { tracksColumns } from '$lib/components/tracks/columns.js';
	import * as Dialog from '$lib/components/ui/dialog/index.js';
	import {
		searchArtists,
		searchAlbums,
		createAdminArtist,
		createAdminAlbum,
		batchUpdateAdminTracks,
	} from '$lib/services/admin-service';
	import type { AdminArtistResponse } from '$lib/bindings/response/admin/admin-artist-response';
	import type { AdminAlbumResponse } from '$lib/bindings/response/admin/admin-album-response';
	import type { TrackResponse } from '$lib/bindings/response/track/track-response';
	import type { RowSelectionState } from '@tanstack/table-core';
	import { toast } from 'svelte-sonner';
	import { goto, invalidateAll } from '$app/navigation';

	let { data } = $props();

	let selectedTracks = $state<TrackResponse[]>([]);
	let trackSelection = $state<RowSelectionState>({});
	let changeArtistOpen = $state(false);
	let changeAlbumOpen = $state(false);
	let batchArtist = $state<AdminArtistResponse | null>(null);
	let batchAlbum = $state<AdminAlbumResponse | null>(null);
	let batchSaving = $state(false);

	async function searchArtistBatch(q: string) {
		const { data } = await searchArtists(fetch, q);
		return data ?? [];
	}
	async function searchAlbumBatch(q: string) {
		const { data } = await searchAlbums(fetch, q);
		return data ?? [];
	}
	async function createArtistBatch(name: string) {
		const { data, error } = await createAdminArtist(fetch, { name });
		if (error) toast.error('Failed to create artist', { description: error });
		return data;
	}
	async function createAlbumBatch(title: string) {
		const { data, error } = await createAdminAlbum(fetch, { title });
		if (error) toast.error('Failed to create album', { description: error });
		return data;
	}

	function clearSelection() {
		trackSelection = {};
		selectedTracks = [];
	}

	async function applyBatchArtist() {
		if (!batchArtist || selectedTracks.length === 0) return;
		batchSaving = true;
		const ids = selectedTracks.map((t) => t.id);
		const { error, data: res } = await batchUpdateAdminTracks(fetch, ids, {
			artistId: batchArtist.id,
		});
		batchSaving = false;
		if (error) {
			toast.error('Batch update failed', { description: error });
			return;
		}
		toast.success(`Updated ${res?.updated ?? ids.length} tracks`);
		changeArtistOpen = false;
		batchArtist = null;
		clearSelection();
		await invalidateAll();
	}

	async function applyBatchAlbum() {
		if (!batchAlbum || selectedTracks.length === 0) return;
		batchSaving = true;
		const ids = selectedTracks.map((t) => t.id);
		const { error, data: res } = await batchUpdateAdminTracks(fetch, ids, {
			albumId: batchAlbum.id,
		});
		batchSaving = false;
		if (error) {
			toast.error('Batch update failed', { description: error });
			return;
		}
		toast.success(`Moved ${res?.updated ?? ids.length} tracks`);
		changeAlbumOpen = false;
		batchAlbum = null;
		clearSelection();
		await invalidateAll();
	}
</script>

<div class="space-y-6 p-6">
	<div class="mx-auto max-w-2xl min-w-md space-y-6">
		<header class="flex items-baseline justify-between gap-2">
			<div>
				<h1>Edit album</h1>
				<p class="text-xs text-muted-foreground">{data.album.id}</p>
			</div>
			<div class="flex items-center gap-1">
				{#if !data.album.approved}
					<span
						class="rounded-md bg-amber-500/10 px-2 py-1 text-xs font-medium text-amber-600 dark:text-amber-400"
					>
						Pending
					</span>
				{/if}
				{#if data.album.lockedAt}
					<span
						class="rounded-md bg-secondary px-2 py-1 text-xs font-medium text-secondary-foreground"
					>
						Locked
					</span>
				{/if}
			</div>
		</header>

		<AlbumEditForm
			album={data.album}
			artist={data.artist}
			canDelete={data.tracks.length === 0}
			onAfterChange={() => invalidateAll()}
			onAfterDelete={() => goto('/admin')}
		/>
	</div>

	<section class="space-y-2">
		<h2>Tracks ({data.tracks.length})</h2>
		{#if data.tracks.length === 0}
			<p class="text-sm text-muted-foreground">No tracks. You can hard-delete this album.</p>
		{:else}
			{#if selectedTracks.length > 0}
				<div
					class="flex flex-wrap items-center gap-2 rounded-md border bg-muted/40 px-3 py-2 text-sm"
				>
					<span class="font-medium">{selectedTracks.length} selected</span>
					<div class="grow"></div>
					<Button
						size="sm"
						variant="outline"
						onclick={() => {
							batchArtist = null;
							changeArtistOpen = true;
						}}
					>
						Change artist
					</Button>
					<Button
						size="sm"
						variant="outline"
						onclick={() => {
							batchAlbum = null;
							changeAlbumOpen = true;
						}}
					>
						Move to album
					</Button>
					<Button size="sm" variant="ghost" onclick={clearSelection}>Clear</Button>
				</div>
			{/if}
			<DataTable
				storageKey="admin:album:tracks"
				data={data.tracks}
				columns={tracksColumns.filter((col) => col.id !== 'album')}
				bind:rowSelection={trackSelection}
				onSelectionChange={(rows) => (selectedTracks = rows as TrackResponse[])}
				onRowClick={(row) => goto(`/admin/tracks/${row.id}`)}
			/>
		{/if}
	</section>
</div>

<Dialog.Root bind:open={changeArtistOpen}>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>Change artist for {selectedTracks.length} tracks</Dialog.Title>
			<Dialog.Description>
				This will set the artist on every selected track and lock those rows from rescan.
			</Dialog.Description>
		</Dialog.Header>
		<EntityPicker
			label="New artist"
			placeholder="Pick an artist…"
			bind:value={batchArtist}
			formatLabel={(a) => a.name}
			search={searchArtistBatch}
			onCreate={createArtistBatch}
		/>
		<Dialog.Footer>
			<Button variant="ghost" onclick={() => (changeArtistOpen = false)}>Cancel</Button>
			<Button onclick={applyBatchArtist} disabled={!batchArtist || batchSaving}>
				{batchSaving ? 'Saving…' : 'Apply'}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>

<Dialog.Root bind:open={changeAlbumOpen}>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>Move {selectedTracks.length} tracks to another album</Dialog.Title>
			<Dialog.Description>
				This will reassign every selected track and lock those rows from rescan.
			</Dialog.Description>
		</Dialog.Header>
		<EntityPicker
			label="Destination album"
			placeholder="Pick an album…"
			bind:value={batchAlbum}
			formatLabel={(a) => a.title}
			search={searchAlbumBatch}
			onCreate={createAlbumBatch}
		/>
		<Dialog.Footer>
			<Button variant="ghost" onclick={() => (changeAlbumOpen = false)}>Cancel</Button>
			<Button onclick={applyBatchAlbum} disabled={!batchAlbum || batchSaving}>
				{batchSaving ? 'Saving…' : 'Apply'}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
