import { extractPaginationFromUrl } from '$lib/utils/pagination';
import type { PageServerLoad } from './$types';
import { getArtists } from '$lib/services/artist-service';
import { error } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ fetch, url }) => {
	const { limit, offset, page } = extractPaginationFromUrl(url);
	const { data: artists, error: errorMessage } = await getArtists(fetch, { limit, offset });

	if (errorMessage) {
		return { artists: null, error: { status: 500, message: errorMessage }, page };
	}
	if (!artists) {
		error(500, 'Failed to load artists');
	}

	return { artists, error: null, page };
};
