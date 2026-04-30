import type { PageServerLoad } from './$types';
import { getTags } from '$lib/services/tag-service';
import { error } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ fetch }) => {
	const { data: tags, error: errorMessage } = await getTags(fetch);

	if (errorMessage) {
		return { tags: null, error: { status: 500, message: errorMessage } };
	}

	if (!tags) {
		error(500, 'Failed to load tags');
	}

	return { tags: tags, error: null };
};
