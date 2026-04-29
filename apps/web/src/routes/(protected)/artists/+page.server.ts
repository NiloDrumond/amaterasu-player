import { extractPaginationFromUrl } from '$lib/utils/pagination';
import type { PageServerLoad } from './$types';
import { getArtists } from '$lib/services/artist-service';

export const load: PageServerLoad = async ({ fetch, url }) => {
	const { limit, offset, page } = extractPaginationFromUrl(url);
	const { data: artists, error } = await getArtists(fetch, { limit, offset });

	if (error) {
		return { artists: null as any, error: { status: 500, message: error }, page };
	}

	return { artists: artists!, error: null, page };
};
