import type { CurrentUserResponse } from '$lib/bindings/response/auth/current-user-response';
import type { SignInEmailParams } from '$lib/bindings/request/auth/sign-in-email-params';
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
