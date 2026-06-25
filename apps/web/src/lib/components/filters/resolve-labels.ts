import type { Field } from '$lib/bindings/filter/field';
import { getTags } from '$lib/services/tag-service';
import { getAlbum } from '$lib/services/album-service';
import { getArtist } from '$lib/services/artist-service';

// Resolves the human-readable label for an id-valued filter leaf (tag/album/
// artist) that arrived without one — e.g. a filter loaded from the URL or a
// saved dynamic playlist, which only carry raw ids.
//
// Lives in a plain module (not the component) so the caches can be ordinary
// Maps. Lookups are deduplicated by caching the in-flight promise per id, so
// the FilterBar effect can call this on every re-run without refetching.

let tagMapPromise: Promise<Map<string, string>> | null = null;

function tagMap(): Promise<Map<string, string>> {
	if (!tagMapPromise) {
		tagMapPromise = getTags(fetch).then(({ data }) => {
			const m = new Map<string, string>();
			for (const t of data ?? []) m.set(t.id, t.name);
			return m;
		});
	}
	return tagMapPromise;
}

const cache = new Map<string, Promise<string | null>>();

async function lookup(field: Field, id: string): Promise<string | null> {
	if (field === 'tag') return (await tagMap()).get(id) ?? null;
	if (field === 'album') return (await getAlbum(fetch, id)).data?.title ?? null;
	if (field === 'artist') return (await getArtist(fetch, id)).data?.name ?? null;
	return null;
}

export function resolveValueLabel(field: Field, id: string): Promise<string | null> {
	const key = `${field}:${id}`;
	let p = cache.get(key);
	if (!p) {
		p = lookup(field, id);
		cache.set(key, p);
	}
	return p;
}
