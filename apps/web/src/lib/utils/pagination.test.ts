import { describe, expect, it } from 'vitest';
import { DEFAULT_LIMIT, extractPaginationFromUrl } from './pagination';

describe('extractPaginationFromUrl', () => {
	it('uses defaults when params are absent', () => {
		const url = new URL('https://example.com/items');
		expect(extractPaginationFromUrl(url)).toEqual({
			limit: DEFAULT_LIMIT,
			offset: 0,
			page: 1,
		});
	});

	it('parses limit and offset', () => {
		const url = new URL('https://example.com/items?limit=10&offset=20');
		expect(extractPaginationFromUrl(url)).toEqual({
			limit: 10,
			offset: 20,
			page: 3,
		});
	});

	it('ignores non-numeric limit and offset', () => {
		const url = new URL('https://example.com/items?limit=foo&offset=bar');
		expect(extractPaginationFromUrl(url)).toEqual({
			limit: DEFAULT_LIMIT,
			offset: 0,
			page: 1,
		});
	});

	it('computes page from offset and limit', () => {
		const url = new URL('https://example.com/items?limit=5&offset=15');
		expect(extractPaginationFromUrl(url).page).toBe(4);
	});
});
