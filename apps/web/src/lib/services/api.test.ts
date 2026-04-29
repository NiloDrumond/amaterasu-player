import { describe, expect, it, vi } from 'vitest';
import { api } from './api';

function makeFetch(status: number, body: unknown): typeof fetch {
	return vi.fn().mockResolvedValue({
		ok: status >= 200 && status < 300,
		status,
		statusText: status === 422 ? 'Unprocessable Entity' : 'Internal Server Error',
		json: () => Promise.resolve(body),
	}) as unknown as typeof fetch;
}

describe('api', () => {
	it('returns data on ok response', async () => {
		const fetch = makeFetch(200, { id: '1', name: 'Test' });
		const result = await api<{ id: string; name: string }>(fetch, '/api/test');
		expect(result).toEqual({ data: { id: '1', name: 'Test' }, error: null, status: 200 });
	});

	it('returns error string on non-ok response with error field in body', async () => {
		const fetch = makeFetch(422, { error: 'missing field' });
		const result = await api(fetch, '/api/test');
		expect(result.data).toBeNull();
		expect(result.error).toBe('missing field');
		expect(result.status).toBe(422);
	});

	it('falls back to statusText when body has no error field', async () => {
		const fetch = makeFetch(500, {});
		const result = await api(fetch, '/api/test');
		expect(result.data).toBeNull();
		expect(result.error).toBe('Internal Server Error');
		expect(result.status).toBe(500);
	});

	it('serialises body as JSON and sets Content-Type header', async () => {
		const mockFetch = vi.fn().mockResolvedValue({
			ok: true,
			status: 200,
			json: () => Promise.resolve(null),
		}) as unknown as typeof fetch;

		await api(mockFetch, '/api/test', { method: 'POST', body: { name: 'hello' } });

		expect(mockFetch).toHaveBeenCalledWith('/api/test', {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify({ name: 'hello' }),
		});
	});

	it('returns network error when fetch throws', async () => {
		const mockFetch = vi
			.fn()
			.mockRejectedValue(new Error('network down')) as unknown as typeof fetch;
		const result = await api(mockFetch, '/api/test');
		expect(result.data).toBeNull();
		expect(result.error).toBe('Network error');
		expect(result.status).toBeNull();
	});
});
