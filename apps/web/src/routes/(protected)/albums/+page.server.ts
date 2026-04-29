import { extractPaginationFromUrl } from '$lib/utils/pagination';
import type { PageServerLoad } from './$types';
import { getAlbums } from '$lib/services/album-service';

export const load: PageServerLoad = async ({ fetch, url }) => {
	const { limit, offset, page } = extractPaginationFromUrl(url);
	const { data: albums, error } = await getAlbums(fetch, { limit, offset });

	if (error) {
		return { albums: null as any, error: { status: 500, message: error }, page };
	}

	return { albums: albums!, error: null, page };
};
