// Force Tailwind to bundle every color/tone we use so var(--color-*) resolves.
// bg-red-300 bg-red-400 bg-red-500 bg-red-600 bg-red-700 bg-red-800
// bg-orange-300 bg-orange-400 bg-orange-500 bg-orange-600 bg-orange-700 bg-orange-800
// bg-amber-300 bg-amber-400 bg-amber-500 bg-amber-600 bg-amber-700 bg-amber-800
// bg-yellow-300 bg-yellow-400 bg-yellow-500 bg-yellow-600 bg-yellow-700 bg-yellow-800
// bg-lime-300 bg-lime-400 bg-lime-500 bg-lime-600 bg-lime-700 bg-lime-800
// bg-green-300 bg-green-400 bg-green-500 bg-green-600 bg-green-700 bg-green-800
// bg-emerald-300 bg-emerald-400 bg-emerald-500 bg-emerald-600 bg-emerald-700 bg-emerald-800
// bg-teal-300 bg-teal-400 bg-teal-500 bg-teal-600 bg-teal-700 bg-teal-800
// bg-cyan-300 bg-cyan-400 bg-cyan-500 bg-cyan-600 bg-cyan-700 bg-cyan-800
// bg-sky-300 bg-sky-400 bg-sky-500 bg-sky-600 bg-sky-700 bg-sky-800
// bg-blue-300 bg-blue-400 bg-blue-500 bg-blue-600 bg-blue-700 bg-blue-800
// bg-indigo-300 bg-indigo-400 bg-indigo-500 bg-indigo-600 bg-indigo-700 bg-indigo-800
// bg-violet-300 bg-violet-400 bg-violet-500 bg-violet-600 bg-violet-700 bg-violet-800
// bg-purple-300 bg-purple-400 bg-purple-500 bg-purple-600 bg-purple-700 bg-purple-800
// bg-fuchsia-300 bg-fuchsia-400 bg-fuchsia-500 bg-fuchsia-600 bg-fuchsia-700 bg-fuchsia-800
// bg-pink-300 bg-pink-400 bg-pink-500 bg-pink-600 bg-pink-700 bg-pink-800
// bg-rose-300 bg-rose-400 bg-rose-500 bg-rose-600 bg-rose-700 bg-rose-800
// bg-slate-300 bg-slate-400 bg-slate-500 bg-slate-600 bg-slate-700 bg-slate-800
// bg-zinc-900

export const TAG_COLOR_NAMES = [
	'red',
	'orange',
	'amber',
	'yellow',
	'lime',
	'green',
	'emerald',
	'teal',
	'cyan',
	'sky',
	'blue',
	'indigo',
	'violet',
	'purple',
	'fuchsia',
	'pink',
	'rose',
	'slate',
] as const;

export const TAG_COLOR_TONES = [300, 400, 500, 600, 700, 800] as const;

export type TagColorName = (typeof TAG_COLOR_NAMES)[number];
export type TagColorTone = (typeof TAG_COLOR_TONES)[number];

/** Resolves a stored token like "teal-500" into a CSS color value. */
export function resolveTagColor(token: string | null | undefined): string | null {
	if (!token) return null;
	const match = /^([a-z]+)-(\d{2,3})$/.exec(token);
	if (!match) return token;
	return `var(--color-${match[1]}-${match[2]})`;
}

/** Best foreground color for a given tag color token. Light tones get dark text. */
export function tagForegroundColor(token: string | null | undefined): string {
	if (!token) return 'inherit';
	const match = /^[a-z]+-(\d{2,3})$/.exec(token);
	if (!match) return 'white';
	const tone = Number(match[1]);
	return tone <= 400 ? 'var(--color-zinc-900)' : 'white';
}
