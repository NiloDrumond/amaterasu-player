<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Field, FieldGroup, FieldLabel } from '$lib/components/ui/field';
	import {
		updateAdminArtist,
		deleteAdminArtist,
		forceRescanArtist,
	} from '$lib/services/admin-service';
	import MergeDialog from '$lib/components/admin/merge-dialog.svelte';
	import DataTable from '$lib/components/ui/data-table/data-table.svelte';
	import { albumsColumns } from '$lib/components/albums/columns.js';
	import { toast } from 'svelte-sonner';
	import { goto, invalidateAll } from '$app/navigation';

	import { untrack } from 'svelte';
	let { data } = $props();
	const initial = untrack(() => data);

	let name = $state(initial.artist.name);
	let sortName = $state(initial.artist.sortName);

	let saving = $state(false);
	let mergeOpen = $state(false);

	async function save() {
		saving = true;
		const { error } = await updateAdminArtist(fetch, data.artist.id, { name, sortName });
		saving = false;
		if (error) toast.error('Save failed', { description: error });
		else {
			toast.success('Saved');
			await invalidateAll();
		}
	}

	async function hardDelete() {
		if (!confirm('Hard-delete this artist? Only allowed if no albums or tracks reference them.'))
			return;
		const { error, status } = await deleteAdminArtist(fetch, data.artist.id);
		if (status === 409) toast.error('Artist is not empty');
		else if (error) toast.error('Delete failed', { description: error });
		else {
			toast.success('Artist deleted');
			goto('/admin');
		}
	}

	async function forceRescan() {
		if (!confirm('Force-rescan? Local edits will be replaced on the next scan.')) return;
		const { error } = await forceRescanArtist(fetch, data.artist.id);
		if (error) toast.error('Failed', { description: error });
		else {
			toast.success('Lock cleared');
			await invalidateAll();
		}
	}
</script>

<div class="space-y-6 p-6">
	<div class="mx-auto max-w-2xl min-w-md space-y-6">
		<header class="flex items-baseline justify-between">
			<div>
				<h1>Edit artist</h1>
				<p class="text-xs text-muted-foreground">{data.artist.id}</p>
			</div>
			{#if data.artist.lockedAt}
				<span
					class="rounded-md bg-secondary px-2 py-1 text-xs font-medium text-secondary-foreground"
				>
					Locked
				</span>
			{/if}
		</header>

		<FieldGroup>
			<Field>
				<FieldLabel for="name">Name</FieldLabel>
				<Input id="name" bind:value={name} />
			</Field>
			<Field>
				<FieldLabel for="sort-name">Sort name</FieldLabel>
				<Input id="sort-name" bind:value={sortName} />
			</Field>

			<div class="border-t pt-3 text-xs text-muted-foreground">
				<div>source_name: <span class="font-mono">{data.artist.sourceName}</span></div>
			</div>
		</FieldGroup>

		<footer class="flex flex-wrap gap-2 border-t pt-4">
			<Button onclick={save} disabled={saving}>{saving ? 'Saving…' : 'Save'}</Button>
			<Button variant="outline" onclick={forceRescan}>Force rescan</Button>
			<Button variant="outline" onclick={() => (mergeOpen = true)}>Merge…</Button>
			<div class="grow"></div>
			<Button variant="destructive" onclick={hardDelete} disabled={data.albums.length > 0}>
				Hard delete
			</Button>
		</footer>
	</div>

	<MergeDialog kind="artist" bind:open={mergeOpen} target={data.artist} />

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
</div>
