import type { PageServerLoad } from './$types';
import { getTags } from '$lib/services/tag-service';

export const load: PageServerLoad = async ({ fetch }) => {
	const { data: tags, error } = await getTags(fetch);

	if (error) {
		return { tags: null as any, error: { status: 500, message: error } };
	}

	return { tags: tags!, error: null };
};
