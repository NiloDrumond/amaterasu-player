import type { PageServerLoad } from './$types';
import { getTags } from '$lib/services/tag-service';
import { getTagCategories } from '$lib/services/tag-category-service';
import { error } from '@sveltejs/kit';

export const load: PageServerLoad = async ({ fetch }) => {
	const [tagsRes, categoriesRes] = await Promise.all([getTags(fetch), getTagCategories(fetch)]);

	const errorMessage = tagsRes.error ?? categoriesRes.error;
	if (errorMessage) {
		return {
			tags: null,
			categories: null,
			error: { status: 500, message: errorMessage },
		};
	}

	if (!tagsRes.data || !categoriesRes.data) {
		error(500, 'Failed to load tags');
	}

	return { tags: tagsRes.data, categories: categoriesRes.data, error: null };
};
