import type { PlayerState } from '$lib/player/player.svelte';
import { useShortcut } from './shortcuts.svelte';

const VOLUME_STEP = 0.05;
const PREV_SEEK_THRESHOLD = 3;

export function usePlayerShortcuts(player: PlayerState): void {
	useShortcut([
		{
			id: 'player.play-pause',
			keys: 'Space',
			handler: () => player.toggle(),
			description: 'Play / pause',
		},
		{
			id: 'player.prev-or-restart',
			keys: 'h',
			handler: () => {
				if (player.currentTime > PREV_SEEK_THRESHOLD) {
					player.currentTime = 0;
				} else {
					player.prev();
				}
			},
			description: 'Previous track / restart',
		},
		{
			id: 'player.next',
			keys: 'l',
			handler: () => player.next(),
			description: 'Next track',
		},
		{
			id: 'player.volume-down',
			keys: 'j',
			handler: () => {
				player.volume = Math.max(0, +(player.volume - VOLUME_STEP).toFixed(2));
			},
			description: 'Volume down',
		},
		{
			id: 'player.volume-up',
			keys: 'k',
			handler: () => {
				player.volume = Math.min(1, +(player.volume + VOLUME_STEP).toFixed(2));
			},
			description: 'Volume up',
		},
		{
			id: 'player.mute',
			keys: 'm',
			handler: () => player.toggleMute(),
			description: 'Mute / unmute',
		},
		{
			id: 'player.repeat',
			keys: 'r',
			handler: () => player.cycleRepeatMode(),
			description: 'Cycle repeat mode',
		},
		{
			id: 'player.shuffle',
			keys: 's',
			handler: () => player.toggleShuffle(),
			description: 'Toggle shuffle',
		},
		{
			id: 'player.focus',
			keys: 'f',
			handler: () => player.toggleFocused(),
			description: 'Toggle focused view',
		},
	]);
}
