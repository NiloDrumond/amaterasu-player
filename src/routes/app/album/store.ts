import {
  NDAlbumListSort,
  type NDAlbumListParams,
  NDSortOrder,
} from '$lib/navidrome/types';
import { createPaginationStore } from '$lib/stores/pagination';
import { scan } from '$lib/stores/scan';
import { get, writable } from 'svelte/store';

const POPULAR_SORT = {
  _sort: NDAlbumListSort.PLAY_COUNT,
  _order: NDSortOrder.DESC,
  recently_played: true,
};
const RANDOM_SORT = {
  _sort: NDAlbumListSort.RANDOM,
  _order: NDSortOrder.ASC,
  recently_played: undefined,
};
const ALPHABETICAL_SORT = {
  _sort: NDAlbumListSort.NAME,
  _order: NDSortOrder.ASC,
  recently_played: undefined,
};
const PAGE_SIZE = 49;
function createAlbumListParams() {
  const store = writable<NDAlbumListParams>({
    _end: PAGE_SIZE,
    ...ALPHABETICAL_SORT,
  });
  const { subscribe, update } = store;
  const pagination = createPaginationStore({
    page: 0,
    pageSize: PAGE_SIZE,
    totalAmount: get(scan)?.totalAlbumCount || 0,
  });

  function setName(value: string) {
    let name: string | undefined = value;
    if (value.trim().length === 0) {
      name = undefined;
    }
    update((prev) => ({ ...prev, name }));
  }

  function setSortAlphabetical() {
    update((prev) => ({
      ...prev,
      ...ALPHABETICAL_SORT,
    }));
    pagination.setPage(0);
  }

  function setSortRandom() {
    const state = get(store);
    const current = state._sort;
    if (current === NDAlbumListSort.RANDOM) {
      setSortAlphabetical();
    } else {
      update((prev) => ({
        ...prev,
        ...RANDOM_SORT,
      }));
      pagination.setPage(0);
    }
  }

  function setSortPopular() {
    const state = get(store);
    const current = state._sort;
    if (current === NDAlbumListSort.PLAY_COUNT) {
      setSortAlphabetical();
    } else {
      update((prev) => ({
        ...prev,
        ...POPULAR_SORT,
      }));
      pagination.setPage(0);
    }
  }

  pagination.subscribe((state) => {
    const _start = state.page * PAGE_SIZE;
    const _end = (state.page + 1) * PAGE_SIZE;
    update((prev) => ({ ...prev, _start, _end }));
  });
  return {
    subscribe,
    setName,
    setSortRandom,
    setSortPopular,
    pagination,
  };
}

export const albumListParams = createAlbumListParams();
