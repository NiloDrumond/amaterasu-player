import { api, type Result } from './api';

type Fetch = typeof fetch;

export function scanLibrary(fetch: Fetch): Promise<Result<void>> {
	return api<void>(fetch, '/api/admin/scan-library', { method: 'POST' });
}
