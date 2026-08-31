import type { PlayerState } from './player.svelte';

const VOLUME_STEP = 0.05;
/** Pixel delta of one mouse-wheel notch in Chromium; Firefox reports lines instead. */
const NOTCH_PX = 100;

/**
 * Builds a `wheel` handler that scrolls the volume, YouTube-Music style.
 *
 * Wheel events reach a window the OS has not focused, so this also covers the
 * common case of scrolling over the player while another app holds focus.
 *
 * Each handler keeps its own accumulator, so create one per container rather
 * than sharing a single instance.
 */
export function createVolumeWheelHandler(player: PlayerState) {
	let accumulated = 0;

	return (event: WheelEvent) => {
		if (event.deltaY === 0) return;
		// Stop the page behind the player from scrolling too.
		event.preventDefault();

		// Line and page deltas already arrive as one discrete notch per event.
		if (event.deltaMode !== 0) {
			player.adjustVolume(event.deltaY < 0 ? VOLUME_STEP : -VOLUME_STEP);
			return;
		}

		// Pixel deltas come from trackpads and from Chromium's mouse wheel, in
		// wildly different sizes, so bank them until a notch's worth has gone by.
		if (accumulated !== 0 && Math.sign(event.deltaY) !== Math.sign(accumulated)) accumulated = 0;
		accumulated += event.deltaY;
		const notches = Math.trunc(accumulated / NOTCH_PX);
		if (notches === 0) return;
		accumulated -= notches * NOTCH_PX;
		player.adjustVolume(-notches * VOLUME_STEP);
	};
}
