import { intervalToDuration } from 'date-fns';

export function formatMilliseconds(ms: number): string {
	const duration = intervalToDuration({ start: 0, end: ms });

	const hours = duration.hours ?? 0;
	const minutes = duration.minutes ?? 0;
	const seconds = duration.seconds ?? 0;

	const pad = (n: number) => String(n).padStart(2, '0');

	if (hours > 0) {
		return `${pad(hours)}:${pad(minutes)}:${pad(seconds)}`;
	}

	return `${pad(minutes)}:${pad(seconds)}`;
}
