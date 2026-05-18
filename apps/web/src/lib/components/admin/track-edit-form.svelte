<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Field, FieldGroup, FieldLabel } from '$lib/components/ui/field';
	import EntityPicker from './entity-picker.svelte';
	import {
		updateAdminTrack,
		softDeleteAdminTrack,
		forceRescanTrack,
		approveTrack,
		unapproveTrack,
		searchArtists,
		searchAlbums,
		createAdminArtist,
		createAdminAlbum,
		type UpdateTrackBody,
	} from '$lib/services/admin-service';
	import type { AdminTrackResponse } from '$lib/bindings/response/admin/admin-track-response';
	import type { AdminArtistResponse } from '$lib/bindings/response/admin/admin-artist-response';
	import type { AdminAlbumResponse } from '$lib/bindings/response/admin/admin-album-response';
	import { toast } from 'svelte-sonner';
	import { untrack } from 'svelte';

	type Props = {
		track: AdminTrackResponse;
		artist: AdminArtistResponse | null;
		album: AdminAlbumResponse | null;
		showArtistLink?: boolean;
		showAlbumLink?: boolean;
		showAlbumPicker?: boolean;
		onAfterChange?: () => void | Promise<void>;
		onAfterDelete?: () => void | Promise<void>;
	};

	let {
		track,
		artist: initialArtist,
		album: initialAlbum,
		showArtistLink = true,
		showAlbumLink = true,
		showAlbumPicker = true,
		onAfterChange,
		onAfterDelete,
	}: Props = $props();

	let title = $state(untrack(() => track.title));
	let sortTitle = $state(untrack(() => track.sortTitle));
	let disc = $state<string | number>(untrack(() => track.disc ?? ''));
	let trackNo = $state<string | number>(untrack(() => track.trackNo ?? ''));
	let date = $state(untrack(() => track.date ?? ''));
	let composer = $state(untrack(() => track.composer ?? ''));
	let comment = $state(untrack(() => track.comment ?? ''));
	let artist = $state<AdminArtistResponse | null>(untrack(() => initialArtist));
	let album = $state<AdminAlbumResponse | null>(untrack(() => initialAlbum));

	const initialArtistId = untrack(() => initialArtist?.id ?? null);
	const initialAlbumId = untrack(() => initialAlbum?.id ?? null);

	let saving = $state(false);
	let approving = $state(false);

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
		const newArtistId = artist?.id ?? null;
		const newAlbumId = album?.id ?? null;
		if (newArtistId !== initialArtistId) body.artistId = newArtistId;
		if (newAlbumId !== initialAlbumId) body.albumId = newAlbumId;

		const { error } = await updateAdminTrack(fetch, track.id, body);
		saving = false;
		if (error) toast.error('Save failed', { description: error });
		else {
			toast.success('Saved');
			await onAfterChange?.();
		}
	}

	async function toggleApproval() {
		approving = true;
		const { error } = track.approved
			? await unapproveTrack(fetch, track.id)
			: await approveTrack(fetch, track.id);
		approving = false;
		if (error) toast.error('Failed', { description: error });
		else {
			toast.success(track.approved ? 'Marked pending' : 'Approved');
			await onAfterChange?.();
		}
	}

	async function softDelete() {
		if (!confirm('Soft-delete this track?')) return;
		const { error } = await softDeleteAdminTrack(fetch, track.id);
		if (error) toast.error('Delete failed', { description: error });
		else {
			toast.success('Track soft-deleted');
			await onAfterDelete?.();
		}
	}

	async function forceRescan() {
		if (
			!confirm(
				'Force-rescan? This clears the lock. Your edits will be replaced by file-tag values on the next scan.',
			)
		)
			return;
		const { error } = await forceRescanTrack(fetch, track.id);
		if (error) toast.error('Failed', { description: error });
		else {
			toast.success('Lock cleared');
			await onAfterChange?.();
		}
	}
</script>

<FieldGroup>
	<Field>
		<FieldLabel for="track-{track.id}-title">Title</FieldLabel>
		<Input id="track-{track.id}-title" bind:value={title} />
	</Field>
	<Field>
		<FieldLabel for="track-{track.id}-sort-title">Sort title</FieldLabel>
		<Input id="track-{track.id}-sort-title" bind:value={sortTitle} />
	</Field>

	<EntityPicker
		label="Artist"
		placeholder="Pick an artist…"
		bind:value={artist}
		formatLabel={(a) => a.name}
		search={searchArtist}
		onCreate={createArtistInline}
	/>
	{#if artist && showArtistLink}
		<Button
			variant="link"
			size="sm"
			class="-mt-2 self-start px-0"
			href="/admin/artists/{artist.id}"
		>
			Edit artist →
		</Button>
	{/if}

	{#if showAlbumPicker}
		<EntityPicker
			label="Album"
			placeholder="Pick an album…"
			bind:value={album}
			formatLabel={(a) => a.title}
			search={searchAlbum}
			onCreate={createAlbumInline}
		/>
		{#if album && showAlbumLink}
			<Button
				variant="link"
				size="sm"
				class="-mt-2 self-start px-0"
				href="/admin/albums/{album.id}"
			>
				Edit album →
			</Button>
		{/if}
	{/if}

	<div class="grid grid-cols-2 gap-3">
		<Field>
			<FieldLabel for="track-{track.id}-disc">Disc</FieldLabel>
			<Input id="track-{track.id}-disc" type="number" bind:value={disc} />
		</Field>
		<Field>
			<FieldLabel for="track-{track.id}-track-no">Track #</FieldLabel>
			<Input id="track-{track.id}-track-no" type="number" bind:value={trackNo} />
		</Field>
	</div>

	<Field>
		<FieldLabel for="track-{track.id}-date">Date (YYYY-MM-DD)</FieldLabel>
		<Input id="track-{track.id}-date" bind:value={date} placeholder="2024-01-15" />
	</Field>
	<Field>
		<FieldLabel for="track-{track.id}-composer">Composer</FieldLabel>
		<Input id="track-{track.id}-composer" bind:value={composer} />
	</Field>
	<Field>
		<FieldLabel for="track-{track.id}-comment">Comment</FieldLabel>
		<Input id="track-{track.id}-comment" bind:value={comment} />
	</Field>

	<div class="border-t pt-3 text-xs text-muted-foreground">
		<div>file_path: <span class="font-mono">{track.filePath}</span></div>
	</div>
</FieldGroup>

<footer class="flex flex-wrap gap-2 border-t pt-4">
	<Button onclick={save} disabled={saving}>{saving ? 'Saving…' : 'Save'}</Button>
	<Button
		variant={track.approved ? 'outline' : 'default'}
		onclick={toggleApproval}
		disabled={approving}
	>
		{track.approved ? 'Unapprove' : 'Approve'}
	</Button>
	<Button variant="outline" onclick={forceRescan}>Force rescan</Button>
	<div class="grow"></div>
	{#if !track.deletedAt}
		<Button variant="destructive" onclick={softDelete}>Soft delete</Button>
	{/if}
</footer>
