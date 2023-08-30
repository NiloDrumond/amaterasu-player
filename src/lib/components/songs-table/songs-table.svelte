<script lang="ts">
  import type { NDSong } from '$lib/navidrome/types';
  import { formatDuration } from '$lib/utils/format';
  import Icon from '@iconify/svelte';
  import IconButton from '../icon-button.svelte';
  import Checkbox from '../checkbox.svelte';
  import { selected } from './store';
  import SelectedToolbar from './selected-toolbar.svelte';
  import { playerQueue } from '$lib/stores/player';

  export let songs: NDSong[];
  export let additionalColumns: Array<'album' | 'artist' | 'playCount'> = [];

  function handleToggleAll() {
    selected.update((prev) => {
      if (prev.length === songs.length) {
        return [];
      } else {
        return songs.map((song) => song.id);
      }
    });
  }

  function handleToggleSong(id: string) {
    if ($selected.includes(id)) {
      selected.update((ids) => ids.filter((i) => i !== id));
    } else {
      selected.update((ids) => [...ids, id]);
    }
  }

  function handleSelectSong(idx: number) {
    playerQueue.setQueue(songs, idx);
  }
</script>

<div>
  {#if $selected.length > 0}
    <SelectedToolbar />
  {/if}
  <table class="w-full border-collapse">
    <thead>
      <tr
        class="bg-slate-200 dark:bg-gray-800 border-b dark:border-gray-950 h-16"
      >
        <th class="w-10 pl-4 text-center"
          ><Checkbox
            checked={$selected.length === songs.length}
            onChange={handleToggleAll}
            key="header"
          /></th
        >
        <th class="w-10 text-center pr-2">#</th>
        <th class="text-start w-[50%]">TITLE</th>
        {#if additionalColumns.includes('artist')}
          <th>ARTIST</th>
        {/if}
        {#if additionalColumns.includes('album')}
          <th>ALBUM</th>
        {/if}
        <th class="w-20"><Icon class="mx-auto" icon="mingcute:time-line" /></th>
        {#if additionalColumns.includes('playCount')}
          <th>PLAYS</th>
        {/if}
        <th class="w-10"> </th>
      </tr>
    </thead>
    <tbody>
      {#each songs as song, index}
        <tr
          on:click={() => handleSelectSong(index)}
          class="cursor-pointer hover:bg-slate-200 hover:bg-opacity-50 dark:hover:bg-gray-700 transition-all"
        >
          <td class="text-center pl-4"
            ><Checkbox
              checked={$selected.includes(song.id)}
              onChange={() => handleToggleSong(song.id)}
              key={song.id}
            /></td
          >
          <td class="text-center pr-2">{song.trackNumber}</td>
          <td>{song.title}</td>
          {#if additionalColumns.includes('artist')}
            <td class="text-center">{song.artist}</td>
          {/if}
          {#if additionalColumns.includes('album')}
            <td>{song.album}</td>
          {/if}
          <td class="text-center w-20">{formatDuration(song.duration)}</td>
          {#if additionalColumns.includes('playCount')}
            <td class="text-center w-20">{song.playCount}</td>
          {/if}
          <td class="text-end pr-4"
            ><IconButton
              class="p-2"
              iconProps={{ width: 20, height: 20 }}
              icon="mingcute:more-1-fill"
            /></td
          >
        </tr>
      {/each}
    </tbody>
  </table>
</div>
