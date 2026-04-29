import { extractPaginationFromUrl } from '$lib/utils/pagination';
import type { PageServerLoad } from './$types';
import { getTracks } from '$lib/services/track-service';

export const load: PageServerLoad = async ({ fetch, url }) => {
	const { limit, offset, page } = extractPaginationFromUrl(url);
	const { data: tracks, error } = await getTracks(fetch, { limit, offset });

	if (error) {
		return { tracks: null as any, error: { status: 500, message: error }, page };
	}

	return { tracks: tracks!, error: null, page };
};
