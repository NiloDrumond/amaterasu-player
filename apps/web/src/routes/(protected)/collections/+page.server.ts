import { error } from '@sveltejs/kit';
import { listCollections } from '$lib/services/album-collection-service';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch }) => {
	const { data, error: errorMessage } = await listCollections(fetch);
	if (errorMessage) {
		return { collections: null, error: { status: 500, message: errorMessage } };
	}
	if (!data) {
		error(500, 'Failed to load collections');
	}
	return { collections: data, error: null };
};
