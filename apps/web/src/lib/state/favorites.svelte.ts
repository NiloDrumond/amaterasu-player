import { SvelteMap } from 'svelte/reactivity';

/**
 * Optimistic favorite state, keyed by track id and shared by every
 * `FavoriteButton` on the page.
 *
 * Track objects reach the buttons as plain server data — inside page loads and
 * player queue items — so a button cannot write its result back anywhere the
 * other buttons would see it. Several buttons routinely render the same track
 * at once (the player bar shows one beside the title and another in the track
 * actions), and the player bar's instances outlive the song they were mounted
 * for. Routing toggles through here keeps every button for a track in agreement
 * and lets each one stay a pure function of its `track` prop.
 *
 * An override outranks the server value for the rest of the session, which is
 * what we want: it always reflects the user's most recent action.
 */
const overrides = new SvelteMap<string, boolean>();

export function isFavorited(track: { id: string; favorite: boolean }): boolean {
	return overrides.get(track.id) ?? track.favorite;
}

export function setFavoriteOverride(id: string, value: boolean) {
	overrides.set(id, value);
}

export function clearFavoriteOverride(id: string) {
	overrides.delete(id);
}
