<script lang="ts">
	import { toast } from 'svelte-sonner';
	import { Button } from '$lib/components/ui/button';
	import { Icons } from '$lib/components/ui/icons';
	import { cn } from '$lib/utils';
	import { favoriteTrack, unfavoriteTrack } from '$lib/services/track-service';
	import type { TrackResponse } from '$lib/bindings/response/track/track-response';

	let { track, class: className }: { track: TrackResponse; class?: string } = $props();

	// Optimistic local state. Where the same instance can outlive a track change
	// (e.g. the player bar), the parent wraps this in `{#key track.id}` so a
	// fresh instance — and a fresh initial value — mounts per song.
	// svelte-ignore state_referenced_locally
	let favorited = $state(track.favorite);
	let pending = $state(false);

	async function toggle(event: MouseEvent) {
		// Prevent row selection / playback when used inside a table row.
		event.stopPropagation();
		if (pending) return;

		const next = !favorited;
		favorited = next;
		pending = true;
		const { error } = next
			? await favoriteTrack(fetch, track.id)
			: await unfavoriteTrack(fetch, track.id);
		pending = false;
		if (error) {
			favorited = !next;
			toast.error(error);
		}
	}
</script>

<Button
	size="icon"
	variant="ghost"
	class={cn(favorited && 'text-primary', className)}
	onclick={toggle}
	aria-label={favorited ? 'Remove from favorites' : 'Add to favorites'}
	aria-pressed={favorited}
>
	<Icons.Heart weight={favorited ? 'fill' : 'regular'} />
</Button>
