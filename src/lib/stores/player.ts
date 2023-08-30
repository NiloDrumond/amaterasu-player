import type { NDSong } from '$lib/navidrome/types';
import { derived, writable } from 'svelte/store';

type AudioPlayerStore = {
  duration: number;
  currentTime: number;
  paused: boolean;
  volume: number;
  muted: boolean;
};
function createAudioPlayer() {
  const { subscribe, set, update } = writable<AudioPlayerStore>({
    duration: 0,
    currentTime: 0,
    paused: true,
    volume: 0.5,
    muted: false,
  });

  function onChangeSong() {
    update((prev) => ({ ...prev, currentTime: 0 }));
  }

  function startPlaying() {
    update((prev) => ({ ...prev, paused: false }));
  }

  return { subscribe, set, update, onChangeSong, startPlaying };
}

export const audioPlayer = createAudioPlayer();
export const effectiveVolume = derived(audioPlayer, ($audioPlayer) => {
  if ($audioPlayer.muted) return 0;
  return $audioPlayer.volume;
});

type PlayerQueueStore = {
  currentSongIdx: number;
  queue: NDSong[];
};
function createPlayerQueue() {
  const { subscribe, set, update } = writable<PlayerQueueStore>({
    queue: [],
    currentSongIdx: -1,
  });

  function setQueue(queue: NDSong[], songIdx?: number) {
    set({ currentSongIdx: songIdx === undefined ? 0 : songIdx, queue: queue });
    audioPlayer.onChangeSong();
    audioPlayer.startPlaying();
  }

  function addToQueue(song: NDSong, position: 'next' | 'later') {
    if (position === 'later') {
      update((prev) => ({ ...prev, queue: [...prev.queue, song] }));
    }
    if (position === 'next') {
      update((prev) => {
        return {
          ...prev,
          queue: [
            ...prev.queue.slice(0, prev.currentSongIdx + 1),
            song,
            ...prev.queue.slice(prev.currentSongIdx + 1),
          ],
        };
      });
    }
  }

  function nextSong() {
    update((prev) => {
      if (prev.currentSongIdx + 1 >= prev.queue.length) {
        return prev;
      }
      audioPlayer.onChangeSong();
      return { ...prev, currentSongIdx: prev.currentSongIdx + 1 };
    });
  }

  function previousSong() {
    update((prev) => {
      if (prev.currentSongIdx === 0) {
        return prev;
      }
      audioPlayer.onChangeSong();
      return { ...prev, currentSongIdx: prev.currentSongIdx - 1 };
    });
  }

  return {
    addToQueue,
    subscribe,
    setQueue,
    nextSong,
    previousSong,
  };
}

export const playerQueue = createPlayerQueue();
export const currentSong = derived(playerQueue, ($playerQueue) => {
  if (
    $playerQueue.currentSongIdx < 0 ||
    $playerQueue.currentSongIdx >= $playerQueue.queue.length
  )
    return undefined;
  return $playerQueue.queue[$playerQueue.currentSongIdx];
});
