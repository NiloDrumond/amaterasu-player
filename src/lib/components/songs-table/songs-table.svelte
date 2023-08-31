<script lang="ts">
  import type { NDSong } from '$lib/navidrome/types';
  import { formatDuration } from '$lib/utils/format';
  import Icon from '@iconify/svelte';
  import Checkbox from '../checkbox.svelte';
  import { selected } from './store';
  import SelectedToolbar from './selected-toolbar.svelte';
  import { currentSong, playerQueue } from '$lib/stores/player-queue';
  import { goto } from '$app/navigation';

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
        class="bg-slate-200 dark:bg-gray-800 border-b dark:border-gray-950 h-12"
      >
        <th class="w-10 pl-4 text-center"
          ><Checkbox
            checked={$selected.length === songs.length}
            onChange={handleToggleAll}
            key="header"
          /></th
        >
        <th class="w-10 text-center pr-2">#</th>
        <th class="text-start w-auto">TITLE</th>
        {#if additionalColumns.includes('album')}
          <th class="w-80">ALBUM</th>
        {/if}
        {#if additionalColumns.includes('artist')}
          <th class="w-60">ARTIST</th>
        {/if}
        <th class="w-20"><Icon class="mx-auto" icon="mingcute:time-line" /></th>
        {#if additionalColumns.includes('playCount')}
          <th>PLAYS</th>
        {/if}
        <th class="w-10"> </th>
      </tr>
    </thead>
    <tbody>
      {#each songs as song, index (song.id)}
        {@const isCurrent = $currentSong?.id === song.id}
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
          <td class={isCurrent ? 'text-crystal' : ''}>{song.title}</td>
          {#if additionalColumns.includes('album')}
            <td class="text-center"
              ><a
                href="/app/album/{song.albumId}"
                on:click={(e) => {
                  e.preventDefault();
                  e.stopPropagation();
                  goto(`/app/album/${song.albumId}`, { replaceState: false });
                }}>{song.album}</a
              ></td
            >
          {/if}
          {#if additionalColumns.includes('artist')}
            <td class="text-center">{song.artist}</td>
          {/if}
          <td class="text-center w-20">{formatDuration(song.duration)}</td>
          {#if additionalColumns.includes('playCount')}
            <td class="text-center w-20">{song.playCount}</td>
          {/if}
          <td class="text-end pr-4">
            <button
              class="ghost !p-1"
              on:click={(e) => {
                e.stopPropagation();
                e.preventDefault();
              }}
            >
              <Icon icon="mingcute:more-1-fill" width={20} height={20} />
            </button>
          </td>
        </tr>
      {/each}
    </tbody>
  </table>
</div>
