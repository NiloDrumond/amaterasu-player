import { extractPaginationFromUrl, extractSortFromUrl } from '$lib/utils/pagination';
import type { PageServerLoad } from './$types';
import { getTracks } from '$lib/services/track-service';
import { error } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ fetch, url }) => {
	const { limit, offset, page } = extractPaginationFromUrl(url);
	const { sort, dir, seed } = extractSortFromUrl(url);
	const f = url.searchParams.get('f');
	const { data: tracks, error: errorMessage } = await getTracks(fetch, {
		limit,
		offset,
		f,
		sort,
		dir,
		seed,
	});

	if (errorMessage) {
		return { tracks: null, error: { status: 500, message: errorMessage }, page };
	}
	if (!tracks) {
		error(500, 'Failed to load tracks');
	}

	return { tracks: tracks, error: null, page };
};
