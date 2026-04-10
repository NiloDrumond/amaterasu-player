import type { CurrentUserResponse } from '$lib/bindings/response/auth/current-user-response';
import type { LayoutServerLoad } from './$types';
import { redirect } from '@sveltejs/kit';

export const load: LayoutServerLoad = async ({ fetch, url }) => {
	const res = await fetch('/api/auth/me');
	if (!res.ok) {
		redirect(303, `/login?redirectTo=${url.pathname}`);
	}
	const user = (await res.json()) as CurrentUserResponse;
	return { user };
};
