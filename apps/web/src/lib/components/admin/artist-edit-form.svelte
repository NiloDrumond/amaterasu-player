<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Field, FieldGroup, FieldLabel } from '$lib/components/ui/field';
	import MergeDialog from './merge-dialog.svelte';
	import {
		updateAdminArtist,
		deleteAdminArtist,
		forceRescanArtist,
		approveArtist,
		unapproveArtist,
	} from '$lib/services/admin-service';
	import type { AdminArtistResponse } from '$lib/bindings/response/admin/admin-artist-response';
	import { toast } from 'svelte-sonner';
	import { untrack } from 'svelte';

	type Props = {
		artist: AdminArtistResponse;
		canDelete?: boolean;
		onAfterChange?: () => void | Promise<void>;
		onAfterDelete?: () => void | Promise<void>;
	};

	let { artist, canDelete = true, onAfterChange, onAfterDelete }: Props = $props();

	let name = $state(untrack(() => artist.name));
	let sortName = $state(untrack(() => artist.sortName));

	let saving = $state(false);
	let approving = $state(false);
	let mergeOpen = $state(false);

	async function save() {
		saving = true;
		const { error } = await updateAdminArtist(fetch, artist.id, { name, sortName });
		saving = false;
		if (error) toast.error('Save failed', { description: error });
		else {
			toast.success('Saved');
			await onAfterChange?.();
		}
	}

	async function toggleApproval() {
		approving = true;
		const { error } = artist.approved
			? await unapproveArtist(fetch, artist.id)
			: await approveArtist(fetch, artist.id);
		approving = false;
		if (error) toast.error('Failed', { description: error });
		else {
			toast.success(artist.approved ? 'Marked pending' : 'Approved');
			await onAfterChange?.();
		}
	}

	async function hardDelete() {
		if (!confirm('Hard-delete this artist? Only allowed if no albums or tracks reference them.'))
			return;
		const { error, status } = await deleteAdminArtist(fetch, artist.id);
		if (status === 409) toast.error('Artist is not empty');
		else if (error) toast.error('Delete failed', { description: error });
		else {
			toast.success('Artist deleted');
			await onAfterDelete?.();
		}
	}

	async function forceRescan() {
		if (!confirm('Force-rescan? Local edits will be replaced on the next scan.')) return;
		const { error } = await forceRescanArtist(fetch, artist.id);
		if (error) toast.error('Failed', { description: error });
		else {
			toast.success('Lock cleared');
			await onAfterChange?.();
		}
	}
</script>

<FieldGroup>
	<Field>
		<FieldLabel for="artist-{artist.id}-name">Name</FieldLabel>
		<Input id="artist-{artist.id}-name" bind:value={name} />
	</Field>
	<Field>
		<FieldLabel for="artist-{artist.id}-sort-name">Sort name</FieldLabel>
		<Input id="artist-{artist.id}-sort-name" bind:value={sortName} />
	</Field>

	<div class="border-t pt-3 text-xs text-muted-foreground">
		<div class="mb-1 font-semibold tracking-wide uppercase">Scan aliases</div>
		{#if artist.aliases.length === 0}
			<div class="italic">none</div>
		{:else}
			<ul class="space-y-0.5">
				{#each artist.aliases as alias (alias.id)}
					<li class="font-mono">{alias.sourceName}</li>
				{/each}
			</ul>
		{/if}
	</div>
</FieldGroup>

<footer class="flex flex-wrap gap-2 border-t pt-4">
	<Button onclick={save} disabled={saving}>{saving ? 'Saving…' : 'Save'}</Button>
	<Button
		variant={artist.approved ? 'outline' : 'default'}
		onclick={toggleApproval}
		disabled={approving}
	>
		{artist.approved ? 'Unapprove' : 'Approve'}
	</Button>
	<Button variant="outline" onclick={forceRescan}>Force rescan</Button>
	<Button variant="outline" onclick={() => (mergeOpen = true)}>Merge…</Button>
	<div class="grow"></div>
	<Button variant="destructive" onclick={hardDelete} disabled={!canDelete}>Hard delete</Button>
</footer>

<MergeDialog kind="artist" bind:open={mergeOpen} target={artist} />
