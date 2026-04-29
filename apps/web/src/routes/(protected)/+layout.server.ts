import type { LayoutServerLoad } from './$types';
import { redirect } from '@sveltejs/kit';
import { getCurrentUser } from '$lib/services/auth-service';

export const load: LayoutServerLoad = async ({ fetch, url }) => {
	const { data: user, error } = await getCurrentUser(fetch);
	if (error) {
		redirect(303, `/login?redirectTo=${url.pathname}`);
	}
	return { user: user! };
};
