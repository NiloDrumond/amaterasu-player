import type { NDSong } from '$lib/navidrome/types';
import { derived, writable } from 'svelte/store';

type PlayerStore = {
  currentSong?: NDSong;
};
function createPlayer() {
  const { subscribe, set } = writable<PlayerStore>({});

  function selectSong(song: NDSong) {
    set({ currentSong: song });
  }

  return {
    subscribe,
    selectSong,
  };
}

export const player = createPlayer();

type PlayerControlsStore = {
  currentSong: NDSong;
};
export const playerControls = derived(player, ($player) => {
  if (!$player.currentSong) {
    throw new Error('no currentSong');
  }
  return $player as PlayerControlsStore;
});
