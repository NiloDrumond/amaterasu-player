import { extractPaginationFromUrl } from '$lib/utils/pagination';
import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { getAlbums } from '$lib/services/album-service';

export const load: PageServerLoad = async ({ fetch, url }) => {
	const { limit, offset, page } = extractPaginationFromUrl(url);
	const { data: albums, error: errorMessage } = await getAlbums(fetch, { limit, offset });

	if (errorMessage) {
		return { albums: null, error: { status: 501, message: errorMessage }, page };
	}
	if (!albums) {
		error(500, 'Failed to load albums');
	}

	return { albums, error: null, page };
};
