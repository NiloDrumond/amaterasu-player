import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';
import { getReviewQueue } from '$lib/services/admin-service';
import { getReviewQueueAlbumSuggestions } from '$lib/services/musicbrainz-service';
import type { MetadataSuggestionResponse } from '$lib/bindings/response/admin/metadata-suggestion-response';

const PAGE_SIZE = 20;

export const load: PageServerLoad = async ({ fetch, url }) => {
	const offset = Math.max(0, Number(url.searchParams.get('offset') ?? '0') || 0);
	const { data, error: err } = await getReviewQueue(fetch, { offset, limit: PAGE_SIZE });
	if (err || !data) error(500, err ?? 'Failed to load review queue');

	// Batch-fetch MB album suggestions for everything on this page. If MB is
	// disabled or errors out, degrade to no suggestions -- the rest of the
	// page still works.
	const albumIds = data.albums.map((g) => g.album.id);
	const { data: suggestionsList } = await getReviewQueueAlbumSuggestions(fetch, albumIds);
	const albumSuggestions: Record<string, MetadataSuggestionResponse[]> = {};
	for (const s of suggestionsList ?? []) {
		(albumSuggestions[s.entityId] ??= []).push(s);
	}

	return { queue: data, offset, pageSize: PAGE_SIZE, albumSuggestions };
};
