import { describe, expect, it } from 'vitest';
import { formatQuality } from './audio';

describe('formatQuality', () => {
	it('returns uppercase format for lossless codec without bitrate', () => {
		expect(formatQuality('flac', 'flac', null)).toBe('FLAC');
	});

	it('returns format only for pcm codecs', () => {
		expect(formatQuality('wav', 'PCM_S16LE', 1_411_200)).toBe('WAV');
	});

	it('includes bitrate in kbps for lossy formats', () => {
		expect(formatQuality('mp3', 'mp3', 320_000)).toBe('MP3 320');
	});

	it('treats null bitrate like lossless for labeling', () => {
		expect(formatQuality('ogg', 'vorbis', null)).toBe('OGG');
	});

	it('normalizes codec case for lossless set', () => {
		expect(formatQuality('flac', 'FLAC', 999_000)).toBe('FLAC');
	});
});
