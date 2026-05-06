import { extractPaginationFromUrl } from '$lib/utils/pagination';
import type { PageServerLoad } from './$types';
import { getTracks } from '$lib/services/track-service';
import { error } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ fetch, url }) => {
	const { limit, offset, page } = extractPaginationFromUrl(url);
	const { data: tracks, error: errorMessage } = await getTracks(fetch, { limit, offset });

	if (errorMessage) {
		return { tracks: null, error: { status: 500, message: errorMessage }, page };
	}
	if (!tracks) {
		error(500, 'Failed to load tracks');
	}

	return { tracks, error: null, page };
};
