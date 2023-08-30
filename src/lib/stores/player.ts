import { ndClient } from '$lib/navidrome/client';
import type { NDSong } from '$lib/navidrome/types';
import { derived, get, writable } from 'svelte/store';

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

type PlayerQueueStore = {
  currentSongIdx: number;
  queue: NDSong[];
  repeat: 'no' | 'all' | 'one';
};
function createPlayerQueue() {
  const store = writable<PlayerQueueStore>({
    queue: [],
    currentSongIdx: -1,
    repeat: 'no',
  });
  const { subscribe, update } = store;

  function setQueue(queue: NDSong[], songIdx?: number) {
    update((prev) => ({
      ...prev,
      currentSongIdx: songIdx === undefined ? 0 : songIdx,
      queue: queue,
    }));
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

  function nextSong(): NDSong | undefined {
    let changed = false;
    update((prev) => {
      if (prev.currentSongIdx + 1 >= prev.queue.length) {
        return prev;
      }
      changed = true;
      audioPlayer.onChangeSong();
      return { ...prev, currentSongIdx: prev.currentSongIdx + 1 };
    });
    if (!changed) return undefined;
    return get(store).queue[get(store).currentSongIdx];
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

export const currentStreamUrl = derived(currentSong, ($currentSong) => {
  if (!$currentSong) return undefined;
  return ndClient.getSongStreamUrl({
    songId: $currentSong.id,
  });
});
