<script lang="ts">
	import { toast } from 'svelte-sonner';
	import { Button } from '$lib/components/ui/button';
	import { Icons } from '$lib/components/ui/icons';
	import { cn } from '$lib/utils';
	import { favoriteTrack, unfavoriteTrack } from '$lib/services/track-service';
	import {
		clearFavoriteOverride,
		isFavorited,
		setFavoriteOverride,
	} from '$lib/state/favorites.svelte';
	import type { TrackResponse } from '$lib/bindings/response/track/track-response';

	let { track, class: className }: { track: TrackResponse; class?: string } = $props();

	// Derived from the prop, so an instance that outlives a track change — the
	// player bar's — follows the new song instead of keeping the old heart.
	// Toggles go to a shared store rather than local state so the other buttons
	// showing the same track update with it.
	const favorited = $derived(isFavorited(track));
	let pending = $state(false);

	async function toggle(event: MouseEvent) {
		// Prevent row selection / playback when used inside a table row.
		event.stopPropagation();
		if (pending) return;

		const id = track.id;
		const next = !favorited;
		setFavoriteOverride(id, next);
		pending = true;
		const { error } = next ? await favoriteTrack(fetch, id) : await unfavoriteTrack(fetch, id);
		pending = false;
		if (error) {
			clearFavoriteOverride(id);
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
