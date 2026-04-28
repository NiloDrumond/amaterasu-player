const LOSSLESS_CODECS = new Set(['flac', 'alac', 'ape', 'wavpack', 'tta', 'tak']);

function isLossless(codec: string): boolean {
	const c = codec.toLowerCase();
	return c.startsWith('pcm_') || LOSSLESS_CODECS.has(c);
}

export function formatQuality(format: string, codec: string, bitrateBps: number | null): string {
	const name = format.toUpperCase();
	if (isLossless(codec) || bitrateBps == null) return name;
	return `${name} ${Math.round(bitrateBps / 1000)}`;
}
