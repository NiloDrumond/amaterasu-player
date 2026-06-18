import type { PlaylistResponse } from '$lib/bindings/response/playlist/playlist-response';
import { getPlaylists } from '$lib/services/playlist-service';

type Fetch = typeof fetch;

const CACHE_TTL_MS = 60 * 1000;

let data = $state<PlaylistResponse[] | null>(null);
let lastFetch = 0;
let inflight: Promise<PlaylistResponse[]> | null = null;

async function fetchAll(fetch: Fetch): Promise<PlaylistResponse[]> {
	const res = await getPlaylists(fetch, { limit: 1000, offset: 0 });
	if (res.error || !res.data) {
		return data ?? [];
	}
	data = res.data.data;
	lastFetch = Date.now();
	return data;
}

/**
 * Returns the cached playlists, fetching them once (and reusing the result for
 * `CACHE_TTL_MS`) so the add-to-playlist dialog feels instant on repeat opens.
 * Concurrent calls share a single in-flight request. Call
 * `invalidatePlaylistsCache()` after any mutation so the next open refetches.
 */
export function loadPlaylists(fetch: Fetch): Promise<PlaylistResponse[]> {
	if (data && Date.now() - lastFetch < CACHE_TTL_MS) {
		return Promise.resolve(data);
	}
	if (inflight) return inflight;
	inflight = fetchAll(fetch).finally(() => {
		inflight = null;
	});
	return inflight;
}

export function invalidatePlaylistsCache() {
	data = null;
	lastFetch = 0;
}
