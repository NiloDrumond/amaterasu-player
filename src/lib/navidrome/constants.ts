import { NDSortOrder, type NDSongListParams, NDSongListSort } from './types';

export const DEFAULT_ALBUM_SONG_LIST_QUERY: NDSongListParams = {
  _start: 0,
  _end: 0,
  _order: NDSortOrder.ASC,
  _sort: NDSongListSort.ALBUM,
};
