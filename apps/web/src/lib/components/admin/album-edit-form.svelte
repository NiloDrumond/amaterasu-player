<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Field, FieldGroup, FieldLabel } from '$lib/components/ui/field';
	import EntityPicker from './entity-picker.svelte';
	import MergeDialog from './merge-dialog.svelte';
	import {
		updateAdminAlbum,
		deleteAdminAlbum,
		forceRescanAlbum,
		approveAlbum,
		unapproveAlbum,
		searchArtists,
		createAdminArtist,
		uploadAlbumCover,
	} from '$lib/services/admin-service';
	import { invalidateAll } from '$app/navigation';
	import type { AdminAlbumResponse } from '$lib/bindings/response/admin/admin-album-response';
	import type { AdminArtistResponse } from '$lib/bindings/response/admin/admin-artist-response';
	import { toast } from 'svelte-sonner';
	import { untrack } from 'svelte';

	type Props = {
		album: AdminAlbumResponse;
		artist: AdminArtistResponse | null;
		canDelete?: boolean;
		showArtistLink?: boolean;
		onAfterChange?: () => void | Promise<void>;
		onAfterDelete?: () => void | Promise<void>;
	};

	let {
		album,
		artist: initialArtist,
		canDelete = true,
		showArtistLink = true,
		onAfterChange,
		onAfterDelete,
	}: Props = $props();

	let title = $state(untrack(() => album.title));
	let sortTitle = $state(untrack(() => album.sortTitle));
	let date = $state(untrack(() => album.date ?? ''));
	let artist = $state<AdminArtistResponse | null>(untrack(() => initialArtist));

	let saving = $state(false);
	let approving = $state(false);
	let mergeOpen = $state(false);
	let uploadingCover = $state(false);

	async function onCoverFileChange(e: Event) {
		const input = e.target as HTMLInputElement;
		const file = input.files?.[0];
		input.value = '';
		if (!file) return;
		uploadingCover = true;
		const { error } = await uploadAlbumCover(fetch, album.id, file);
		uploadingCover = false;
		if (error) {
			toast.error('Cover upload failed', { description: error });
		} else {
			toast.success('Cover updated');
			await invalidateAll();
			await onAfterChange?.();
		}
	}

	async function searchArtist(q: string) {
		const { data } = await searchArtists(fetch, q);
		return data ?? [];
	}
	async function createArtistInline(name: string) {
		const { data, error } = await createAdminArtist(fetch, { name });
		if (error) toast.error('Failed to create artist', { description: error });
		return data;
	}

	async function save() {
		saving = true;
		const { error } = await updateAdminAlbum(fetch, album.id, {
			title,
			sortTitle,
			artistId: artist?.id ?? null,
			date: date || null,
		});
		saving = false;
		if (error) toast.error('Save failed', { description: error });
		else {
			toast.success('Saved');
			await onAfterChange?.();
		}
	}

	async function toggleApproval() {
		approving = true;
		const { error } = album.approved
			? await unapproveAlbum(fetch, album.id)
			: await approveAlbum(fetch, album.id);
		approving = false;
		if (error) toast.error('Failed', { description: error });
		else {
			toast.success(album.approved ? 'Marked pending' : 'Approved');
			await onAfterChange?.();
		}
	}

	async function hardDelete() {
		if (!confirm('Hard-delete this album? Only allowed if it has no tracks.')) return;
		const { error, status } = await deleteAdminAlbum(fetch, album.id);
		if (status === 409) toast.error('Album still has tracks');
		else if (error) toast.error('Delete failed', { description: error });
		else {
			toast.success('Album deleted');
			await onAfterDelete?.();
		}
	}

	async function forceRescan() {
		if (!confirm('Force-rescan? Local edits will be replaced on the next scan.')) return;
		const { error } = await forceRescanAlbum(fetch, album.id);
		if (error) toast.error('Failed', { description: error });
		else {
			toast.success('Lock cleared');
			await onAfterChange?.();
		}
	}
</script>

<FieldGroup>
	<Field>
		<FieldLabel>Cover</FieldLabel>
		<label
			class="group relative block aspect-square size-24 cursor-pointer overflow-hidden rounded border bg-muted w-max!"
			title="Click to upload a new cover"
		>
			{#if album.coverUrl}
				<img src={album.coverUrl} alt="cover for {album.title}" class="size-full object-cover" />
			{:else}
				<span
					class="flex size-full items-center justify-center text-[10px] font-medium tracking-wide text-muted-foreground uppercase"
				>
					No cover
				</span>
			{/if}
			<span
				class="absolute inset-0 flex items-center justify-center bg-black/40 text-xs font-semibold text-white opacity-0 transition group-hover:opacity-100"
			>
				{uploadingCover ? 'Uploading…' : 'Upload'}
			</span>
			<input
				type="file"
				accept="image/jpeg,image/png,image/webp"
				class="sr-only"
				disabled={uploadingCover}
				onchange={onCoverFileChange}
			/>
		</label>
	</Field>
	<Field>
		<FieldLabel for="album-{album.id}-title">Title</FieldLabel>
		<Input id="album-{album.id}-title" bind:value={title} />
	</Field>
	<Field>
		<FieldLabel for="album-{album.id}-sort-title">Sort title</FieldLabel>
		<Input id="album-{album.id}-sort-title" bind:value={sortTitle} />
	</Field>

	<EntityPicker
		label="Album artist"
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

	<Field>
		<FieldLabel for="album-{album.id}-date">Date (YYYY-MM-DD)</FieldLabel>
		<Input id="album-{album.id}-date" bind:value={date} placeholder="2024-01-15" />
	</Field>

	<div class="border-t pt-3 text-xs text-muted-foreground">
		<div class="mb-1 font-semibold tracking-wide uppercase">Scan aliases</div>
		{#if album.aliases.length === 0}
			<div class="italic">none</div>
		{:else}
			<ul class="space-y-0.5">
				{#each album.aliases as alias (alias.id)}
					<li class="font-mono">
						{alias.sourceTitle}{#if alias.sourceAlbumArtistId}
							<span class="text-muted-foreground/70"> · {alias.sourceAlbumArtistId}</span>
						{/if}
					</li>
				{/each}
			</ul>
		{/if}
	</div>
</FieldGroup>

<footer class="flex flex-wrap gap-2 border-t pt-4">
	<Button onclick={save} disabled={saving}>{saving ? 'Saving…' : 'Save'}</Button>
	<Button
		variant={album.approved ? 'outline' : 'default'}
		onclick={toggleApproval}
		disabled={approving}
	>
		{album.approved ? 'Unapprove' : 'Approve'}
	</Button>
	<Button variant="outline" onclick={forceRescan}>Force rescan</Button>
	<Button variant="outline" onclick={() => (mergeOpen = true)}>Merge…</Button>
	<div class="grow"></div>
	<Button variant="destructive" onclick={hardDelete} disabled={!canDelete}>Hard delete</Button>
</footer>

<MergeDialog kind="album" bind:open={mergeOpen} target={album} targetArtist={artist} />
