import { error } from '@sveltejs/kit';
import { extractPaginationFromUrl } from '$lib/utils/pagination';
import { getCollection, getCollectionAlbums } from '$lib/services/album-collection-service';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, params, url }) => {
	const { limit, offset, page } = extractPaginationFromUrl(url);
	const [collectionRes, albumsRes] = await Promise.all([
		getCollection(fetch, params.id),
		getCollectionAlbums(fetch, params.id, { limit, offset }),
	]);

	if (collectionRes.status === 404) {
		error(404, 'Collection not found');
	}
	if (collectionRes.error || albumsRes.error || !collectionRes.data || !albumsRes.data) {
		error(500, collectionRes.error ?? albumsRes.error ?? 'Failed to load collection');
	}

	return {
		collection: collectionRes.data,
		albums: albumsRes.data,
		page,
	};
};
