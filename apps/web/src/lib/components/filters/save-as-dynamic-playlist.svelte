<script lang="ts">
	import { goto } from '$app/navigation';
	import * as Popover from '$lib/components/ui/popover/index.js';
	import { Button } from '$lib/components/ui/button/index.js';
	import { Input } from '$lib/components/ui/input/index.js';
	import { Icons } from '$lib/components/ui/icons';
	import { createPlaylist } from '$lib/services/playlist-service';
	import { invalidatePlaylistsCache } from '$lib/state/playlists-cache.svelte';
	import { toast } from 'svelte-sonner';
	import type { FilterNode } from '$lib/bindings/filter/filter-node';

	let {
		filter,
	}: {
		filter: FilterNode | null;
	} = $props();

	let open = $state(false);
	let name = $state('');
	let saving = $state(false);

	async function save() {
		if (!filter || !name.trim()) return;
		saving = true;
		const { data, error } = await createPlaylist(fetch, {
			name: name.trim(),
			filterDefinition: filter,
		});
		saving = false;
		if (error || !data) {
			toast.error(error ?? 'Failed to save playlist');
			return;
		}
		invalidatePlaylistsCache();
		toast.success(`Saved as "${data.name}"`);
		open = false;
		name = '';
		goto(`/playlists/${data.id}`);
	}
</script>

{#if filter && filter.kind === 'group' && filter.children.length > 0}
	<Popover.Root bind:open>
		<Popover.Trigger>
			{#snippet child({ props })}
				<Button {...props} variant="ghost" size="sm" class="h-7 gap-1 px-2 text-xs">
					<Icons.Sparkle class="size-3" />
					Save as playlist
				</Button>
			{/snippet}
		</Popover.Trigger>
		<Popover.Content class="w-64 p-3" align="end">
			<div class="flex flex-col gap-2">
				<div class="text-xs font-medium">Save filters as a dynamic playlist</div>
				<Input
					bind:value={name}
					placeholder="Playlist name"
					autofocus
					onkeydown={(e: KeyboardEvent) => {
						if (e.key === 'Enter') {
							e.preventDefault();
							save();
						}
					}}
				/>
				<Button size="sm" onclick={save} disabled={!name.trim() || saving}>
					{saving ? 'Saving…' : 'Save'}
				</Button>
			</div>
		</Popover.Content>
	</Popover.Root>
{/if}
