import { extractPaginationFromUrl } from '$lib/utils/pagination';
import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { getAlbums } from '$lib/services/album-service';
import type { GetAlbumsResponse } from '$lib/bindings/response/album/get-albums-response';

type LoadData =
	| {
			albums: GetAlbumsResponse;
			error: null;
			page: number;
	  }
	| {
			albums: null;
			error: { status: number; message: string };
			page: number;
	  };

export const load: PageServerLoad = async ({ fetch, url }): Promise<LoadData> => {
	const { limit, offset, page } = extractPaginationFromUrl(url);
	const f = url.searchParams.get('f');
	const { data: albums, error: errorMessage } = await getAlbums(fetch, { limit, offset, f });

	if (errorMessage) {
		return { albums: null, error: { status: 501, message: errorMessage }, page };
	}

	if (!albums) {
		error(500, 'Failed to load albums');
	}

	return { albums: albums, error: null, page };
};
