import { extractPaginationFromUrl, extractSortFromUrl } from '$lib/utils/pagination';
import type { PageServerLoad } from './$types';
import { getPlaylists } from '$lib/services/playlist-service';
import { error } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ fetch, url }) => {
	const { limit, offset, page } = extractPaginationFromUrl(url);
	const { sort, dir } = extractSortFromUrl(url);
	const q = url.searchParams.get('q');
	const { data: playlists, error: errorMessage } = await getPlaylists(fetch, {
		limit,
		offset,
		q,
		sort,
		dir,
	});

	if (errorMessage) {
		return { playlists: null, error: { status: 500, message: errorMessage }, page };
	}
	if (!playlists) {
		error(500, 'Failed to load playlists');
	}

	return { playlists, error: null, page };
};
