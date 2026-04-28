import { describe, expect, it } from 'vitest';
import { formatMilliseconds } from './date';

describe('formatMilliseconds', () => {
	it('formats zero', () => {
		expect(formatMilliseconds(0)).toBe('00:00');
	});

	it('formats sub-hour duration', () => {
		expect(formatMilliseconds(65_000)).toBe('01:05');
	});

	it('pads seconds and minutes', () => {
		expect(formatMilliseconds(9_000)).toBe('00:09');
	});

	it('includes hours when non-zero', () => {
		expect(formatMilliseconds(3_661_000)).toBe('01:01:01');
	});
});
