<script lang="ts">
	import { Button } from '$lib/components/ui/button';
	import {
		restoreAdminTrack,
		forceRescanTrack,
		hardDeleteAdminTrack,
	} from '$lib/services/admin-service';
	import { toast } from 'svelte-sonner';
	import { invalidateAll } from '$app/navigation';

	let { data } = $props();

	async function restore(id: string) {
		const { error } = await restoreAdminTrack(fetch, id);
		if (error) toast.error('Restore failed', { description: error });
		else {
			toast.success('Restored');
			await invalidateAll();
		}
	}

	async function forceRescan(id: string) {
		const { error } = await forceRescanTrack(fetch, id);
		if (error) toast.error('Failed', { description: error });
		else {
			toast.success('Lock cleared — next scan repopulates');
			await invalidateAll();
		}
	}

	async function hardDelete(id: string) {
		if (!confirm('Permanently delete? This cannot be undone.')) return;
		const { error } = await hardDeleteAdminTrack(fetch, id);
		if (error) toast.error('Delete failed', { description: error });
		else {
			toast.success('Permanently deleted');
			await invalidateAll();
		}
	}
</script>

<div class="mx-auto max-w-4xl space-y-4 p-6">
	<header>
		<h1>Deleted tracks</h1>
		<p class="text-sm text-muted-foreground">
			Soft-deleted tracks survive rescans. Restore brings them back as-is. Force rescan clears the
			lock so file-tag values come back. Hard delete removes the row permanently.
		</p>
	</header>

	{#if data.tracks.length === 0}
		<p class="text-sm text-muted-foreground">Nothing here.</p>
	{:else}
		<table class="w-full text-sm">
			<thead class="text-xs text-muted-foreground">
				<tr class="border-b">
					<th class="py-2 text-left">Title</th>
					<th class="py-2 text-left">Deleted</th>
					<th class="py-2 text-left">File</th>
					<th class="py-2 text-right">Actions</th>
				</tr>
			</thead>
			<tbody>
				{#each data.tracks as t (t.id)}
					<tr class="border-b border-border/50">
						<td class="py-2">{t.title}</td>
						<td class="py-2 text-xs text-muted-foreground">
							{t.deletedAt ? new Date(t.deletedAt).toLocaleString() : '—'}
						</td>
						<td class="py-2 text-xs">
							{#if t.fileMissing}
								<span class="rounded bg-red-500/10 px-1.5 py-0.5 text-red-500">missing</span>
							{:else}
								<span class="text-muted-foreground">on disk</span>
							{/if}
						</td>
						<td class="flex justify-end gap-1 py-2">
							<Button size="sm" variant="outline" onclick={() => restore(t.id)}>Restore</Button>
							<Button size="sm" variant="outline" onclick={() => forceRescan(t.id)}>
								Force rescan
							</Button>
							<Button size="sm" variant="destructive" onclick={() => hardDelete(t.id)}>
								Delete
							</Button>
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
	{/if}
</div>
