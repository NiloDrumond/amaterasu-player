import type { CurrentUserResponse } from '$lib/bindings/response/auth/current-user-response';
import type { SignInEmailParams } from '$lib/bindings/request/auth/sign-in-email-params';
import type { UserPreferences } from '$lib/bindings/response/user/user-preferences';
import { api, type Result } from './api';

type Fetch = typeof fetch;

export function getCurrentUser(fetch: Fetch): Promise<Result<CurrentUserResponse>> {
	return api<CurrentUserResponse>(fetch, '/api/auth/me');
}

export function signIn(
	fetch: Fetch,
	params: SignInEmailParams,
): Promise<Result<CurrentUserResponse>> {
	return api<CurrentUserResponse>(fetch, '/api/auth/sign-in', { method: 'POST', body: params });
}

export function signOut(fetch: Fetch): Promise<Result<void>> {
	return api<void>(fetch, '/api/auth/sign-out', { method: 'POST' });
}

export function updatePreferences(
	fetch: Fetch,
	preferences: UserPreferences,
): Promise<Result<void>> {
	return api<void>(fetch, '/api/auth/me/preferences', {
		method: 'PUT',
		body: { preferences },
	});
}
