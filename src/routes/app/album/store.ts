import type { NDAlbumListParams } from '$lib/navidrome/types';
import { writable } from 'svelte/store';

function createAlbumListParams() {
  const { subscribe, update } = writable<NDAlbumListParams>({});

  function setName(value: string) {
    let name: string | undefined = value;
    if (value.trim().length === 0) {
      name = undefined;
    }
    update((prev) => ({ ...prev, name }));
  }

  return {
    subscribe,
    setName,
  };
}

export const albumListParams = createAlbumListParams();
