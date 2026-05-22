import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';
import { getReviewQueue } from '$lib/services/admin-service';
import { getReviewQueueMbSuggestions } from '$lib/services/musicbrainz-service';
import type { MetadataSuggestionResponse } from '$lib/bindings/response/admin/metadata-suggestion-response';

const PAGE_SIZE = 20;

export const load: PageServerLoad = async ({ fetch, url }) => {
	const offset = Math.max(0, Number(url.searchParams.get('offset') ?? '0') || 0);
	const { data, error: err } = await getReviewQueue(fetch, { offset, limit: PAGE_SIZE });
	if (err || !data) error(500, err ?? 'Failed to load review queue');

	// Collect every album + artist on this page so we can batch-fetch all
	// their pending MB suggestions in one round-trip. If MB is disabled or
	// errors out, degrade to no suggestions -- the rest of the page still
	// works.
	const albumIds = data.albums.map((g) => g.album.id);
	const artistIdSet = new Set<string>();
	for (const g of data.albums) {
		if (g.artist) artistIdSet.add(g.artist.id);
		for (const a of g.trackArtists) artistIdSet.add(a.id);
	}
	for (const g of data.standaloneArtists) artistIdSet.add(g.artist.id);
	const artistIds = [...artistIdSet];

	const { data: suggestionsList } = await getReviewQueueMbSuggestions(fetch, {
		albumIds,
		artistIds,
	});

	const albumSuggestions: Record<string, MetadataSuggestionResponse[]> = {};
	const artistSuggestions: Record<string, MetadataSuggestionResponse[]> = {};
	for (const s of suggestionsList ?? []) {
		const bucket = s.entityType === 'album' ? albumSuggestions : artistSuggestions;
		(bucket[s.entityId] ??= []).push(s);
	}

	return {
		queue: data,
		offset,
		pageSize: PAGE_SIZE,
		albumSuggestions,
		artistSuggestions,
	};
};
