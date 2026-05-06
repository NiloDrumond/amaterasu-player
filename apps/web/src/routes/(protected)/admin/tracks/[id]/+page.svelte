<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Field, FieldGroup, FieldLabel } from '$lib/components/ui/field';
	import EntityPicker from '$lib/components/admin/entity-picker.svelte';
	import {
		updateAdminTrack,
		softDeleteAdminTrack,
		forceRescanTrack,
		searchArtists,
		searchAlbums,
		createAdminArtist,
		createAdminAlbum,
		type UpdateTrackBody,
	} from '$lib/services/admin-service';
	import type { AdminArtistResponse } from '$lib/bindings/response/admin/admin-artist-response';
	import type { AdminAlbumResponse } from '$lib/bindings/response/admin/admin-album-response';
	import { toast } from 'svelte-sonner';
	import { invalidateAll } from '$app/navigation';

	import { untrack } from 'svelte';
	let { data } = $props();
	const initial = untrack(() => data);

	let title = $state(initial.track.title);
	let sortTitle = $state(initial.track.sortTitle);
	let disc = $state<string | number>(initial.track.disc ?? '');
	let trackNo = $state<string | number>(initial.track.trackNo ?? '');
	let date = $state(initial.track.date ?? '');
	let composer = $state(initial.track.composer ?? '');
	let comment = $state(initial.track.comment ?? '');
	let artist = $state<AdminArtistResponse | null>(initial.artist);
	let album = $state<AdminAlbumResponse | null>(initial.album);

	const initialArtistId = initial.artist?.id ?? null;
	const initialAlbumId = initial.album?.id ?? null;

	let saving = $state(false);

	async function searchArtist(q: string) {
		const { data } = await searchArtists(fetch, q);
		return data ?? [];
	}
	async function searchAlbum(q: string) {
		const { data } = await searchAlbums(fetch, q, { artistId: artist?.id });
		return data ?? [];
	}
	async function createArtistInline(name: string) {
		const { data, error } = await createAdminArtist(fetch, { name });
		if (error) toast.error('Failed to create artist', { description: error });
		return data;
	}
	async function createAlbumInline(title: string) {
		const { data, error } = await createAdminAlbum(fetch, {
			title,
			artistId: artist?.id ?? null,
		});
		if (error) toast.error('Failed to create album', { description: error });
		return data;
	}

	function nullableInt(s: string | number): number | null {
		if (typeof s === 'number') return Number.isFinite(s) ? Math.trunc(s) : null;
		const t = s.trim();
		if (!t) return null;
		const n = Number(t);
		return Number.isFinite(n) ? Math.trunc(n) : null;
	}

	async function save() {
		saving = true;
		const body: UpdateTrackBody = {
			title,
			sortTitle,
			disc: nullableInt(disc),
			trackNo: nullableInt(trackNo),
			date: date || null,
			composer: composer || null,
			comment: comment || null,
		};
		// Include FK fields only when changed (covers reassign + clear)
		const newArtistId = artist?.id ?? null;
		const newAlbumId = album?.id ?? null;
		if (newArtistId !== initialArtistId) body.artistId = newArtistId;
		if (newAlbumId !== initialAlbumId) body.albumId = newAlbumId;

		const { error } = await updateAdminTrack(fetch, data.track.id, body);
		saving = false;
		if (error) toast.error('Save failed', { description: error });
		else {
			toast.success('Saved');
			await invalidateAll();
		}
	}

	async function softDelete() {
		if (!confirm('Soft-delete this track?')) return;
		const { error } = await softDeleteAdminTrack(fetch, data.track.id);
		if (error) toast.error('Delete failed', { description: error });
		else {
			toast.success('Track soft-deleted');
			await invalidateAll();
		}
	}

	async function forceRescan() {
		if (
			!confirm(
				'Force-rescan? This clears the lock. Your edits will be replaced by file-tag values on the next scan.',
			)
		)
			return;
		const { error } = await forceRescanTrack(fetch, data.track.id);
		if (error) toast.error('Failed', { description: error });
		else {
			toast.success('Lock cleared');
			await invalidateAll();
		}
	}
</script>

<div class="mx-auto max-w-2xl min-w-md space-y-6 p-6">
	<header class="flex items-baseline justify-between">
		<div>
			<h1>Edit track</h1>
			<p class="text-xs text-muted-foreground">{data.track.id}</p>
		</div>
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
	</header>

	<FieldGroup>
		<Field>
			<FieldLabel for="title">Title</FieldLabel>
			<Input id="title" bind:value={title} />
		</Field>
		<Field>
			<FieldLabel for="sort-title">Sort title</FieldLabel>
			<Input id="sort-title" bind:value={sortTitle} />
		</Field>

		<EntityPicker
			label="Artist"
			placeholder="Pick an artist…"
			bind:value={artist}
			formatLabel={(a) => a.name}
			search={searchArtist}
			onCreate={createArtistInline}
		/>
		{#if artist}
			<Button
				variant="link"
				size="sm"
				class="-mt-2 self-start px-0"
				href="/admin/artists/{artist.id}"
			>
				Edit artist →
			</Button>
		{/if}

		<EntityPicker
			label="Album"
			placeholder="Pick an album…"
			bind:value={album}
			formatLabel={(a) => a.title}
			search={searchAlbum}
			onCreate={createAlbumInline}
		/>
		{#if album}
			<Button
				variant="link"
				size="sm"
				class="-mt-2 self-start px-0"
				href="/admin/albums/{album.id}"
			>
				Edit album →
			</Button>
		{/if}

		<div class="grid grid-cols-2 gap-3">
			<Field>
				<FieldLabel for="disc">Disc</FieldLabel>
				<Input id="disc" type="number" bind:value={disc} />
			</Field>
			<Field>
				<FieldLabel for="track-no">Track #</FieldLabel>
				<Input id="track-no" type="number" bind:value={trackNo} />
			</Field>
		</div>

		<Field>
			<FieldLabel for="date">Date (YYYY-MM-DD)</FieldLabel>
			<Input id="date" bind:value={date} placeholder="2024-01-15" />
		</Field>
		<Field>
			<FieldLabel for="composer">Composer</FieldLabel>
			<Input id="composer" bind:value={composer} />
		</Field>
		<Field>
			<FieldLabel for="comment">Comment</FieldLabel>
			<Input id="comment" bind:value={comment} />
		</Field>

		<div class="border-t pt-3 text-xs text-muted-foreground">
			<div>file_path: <span class="font-mono">{data.track.filePath}</span></div>
		</div>
	</FieldGroup>

	<footer class="flex flex-wrap gap-2 border-t pt-4">
		<Button onclick={save} disabled={saving}>{saving ? 'Saving…' : 'Save'}</Button>
		<Button variant="outline" onclick={forceRescan}>Force rescan</Button>
		<div class="grow"></div>
		{#if !data.track.deletedAt}
			<Button variant="destructive" onclick={softDelete}>Soft delete</Button>
		{/if}
	</footer>
</div>
