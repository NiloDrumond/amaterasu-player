import type { TagResponse } from '$lib/bindings/response/tag/tag-response';
import { getTags } from '$lib/services/tag-service';

type Fetch = typeof fetch;

const CACHE_TTL_MS = 60 * 1000;

let data = $state<TagResponse[] | null>(null);
let lastFetch = 0;
let inflight: Promise<TagResponse[]> | null = null;

async function fetchAll(fetch: Fetch): Promise<TagResponse[]> {
	const res = await getTags(fetch);
	if (res.error || !res.data) {
		return data ?? [];
	}
	data = res.data;
	lastFetch = Date.now();
	return data;
}

/**
 * Returns the cached tags, fetching them once (and reusing the result for
 * `CACHE_TTL_MS`) so the tag picker opens instantly. Concurrent calls share a
 * single in-flight request. Call `invalidateTagsCache()` after any mutation so
 * the next read refetches.
 */
export function loadTags(fetch: Fetch): Promise<TagResponse[]> {
	if (data && Date.now() - lastFetch < CACHE_TTL_MS) {
		return Promise.resolve(data);
	}
	if (inflight) return inflight;
	inflight = fetchAll(fetch).finally(() => {
		inflight = null;
	});
	return inflight;
}

export function invalidateTagsCache() {
	data = null;
	lastFetch = 0;
}
