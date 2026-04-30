import { extractPaginationFromUrl } from '$lib/utils/pagination';
import type { PageServerLoad } from './$types';
import { getArtists } from '$lib/services/artist-service';
import type { GetArtistsResponse } from '$lib/bindings/response/artist/get-artists-response';
import { error } from '@sveltejs/kit';

type LoadData =
	| {
			artists: GetArtistsResponse;
			error: null;
			page: number;
	  }
	| {
			artists: null;
			error: { status: number; message: string };
			page: number;
	  };
export const load: PageServerLoad = async ({ fetch, url }): Promise<LoadData> => {
	const { limit, offset, page } = extractPaginationFromUrl(url);
	const { data: artists, error: errorMessage } = await getArtists(fetch, { limit, offset });

	if (errorMessage) {
		return { artists: null, error: { status: 500, message: errorMessage }, page };
	}

	if (!artists) {
		error(500, 'Failed to load artists');
	}

	return { artists: artists, error: null, page };
};
