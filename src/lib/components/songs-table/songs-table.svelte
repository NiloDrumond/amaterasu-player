<script lang="ts">
  import type { NDSong } from '$lib/navidrome/types';
  import { formatDuration } from '$lib/utils/format';
  import Icon from '@iconify/svelte';
  import IconButton from '../icon-button.svelte';
  import Checkbox from '../checkbox.svelte';
  import { selected } from './store';
  import TextButton from '../text-button.svelte';

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

  function handleDeselectAll() {
    selected.set([]);
  }

  function handleToggleSong(id: string) {
    if ($selected.includes(id)) {
      selected.update((ids) => ids.filter((i) => i !== id));
    } else {
      selected.update((ids) => [...ids, id]);
    }
  }
</script>

<div>
  {#if $selected.length > 0}
    <div
      class={'sticky h-0 z-10 top-20  w-full  left-0 flex flex-row gap-2 items-center'}
    >
      <div class="relative w-full">
        <div
          class="absolute h-20 -top-20 w-full flex flex-row items-center bg-crystal-primary rounded-tl-lg rounded-tr-lg py-4 px-2 gap-2"
        >
          <IconButton size={20} icon="ph:x" on:click={handleDeselectAll} />
          <p>{$selected.length} items selected</p>
          <div class="flex-1" />
          <TextButton
            class="font-normal"
            size={16}
            label="PLAY NOW"
            icon="mingcute:play-fill"
          />
          <TextButton
            class="font-normal"
            size={16}
            label="PLAY NEXT"
            icon="iconamoon:playlist"
          />
          <TextButton
            class="font-normal"
            size={16}
            label="PLAY LATER"
            icon="ph:list-plus-bold"
          />
          <TextButton
            class="font-normal"
            size={16}
            label="ADD TO PLAYLIST"
            icon="ph:music-notes-plus-bold"
          />
        </div>
      </div>
    </div>
  {/if}
  <table class="w-full border-collapse">
    <thead>
      <tr class="bg-gray-700 border-b border-crystal-primary h-16">
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
        <th> </th>
      </tr>
    </thead>
    <tbody>
      {#each songs as song}
        <tr class="cursor-pointer hover:bg-gray-500 transition-all">
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
