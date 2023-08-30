<script lang="ts">
  import {
    dndzone,
    type DndEvent,
    type TransformDraggedElementFunction,
  } from 'svelte-dnd-action';
  import { flip } from 'svelte/animate';
  const flipDurationMs = 200;

  import { currentSong, playerQueue } from '$lib/stores/player-queue';
  import Icon from '@iconify/svelte';
  import type { NDSong } from '$lib/navidrome/types';
  import { showPlayQueue } from '$lib/stores/ui';
  let renderedList: NDSong[];
  $: renderedList = [...$playerQueue.queue];
  function handleConsider(e: CustomEvent<DndEvent<NDSong>>) {
    renderedList = e.detail.items;
  }
  function handleSort(e: CustomEvent<DndEvent<NDSong>>) {
    playerQueue.reorderQueue(e.detail.items);
  }
  function transformDraggedElement(
    args: Parameters<TransformDraggedElementFunction>,
  ) {
    const [draggedEl, data, index] = args;
    if (!draggedEl) return;
    const el = draggedEl.querySelector('.draggable') as HTMLButtonElement;
    el.style.cursor = 'grab';
  }
</script>

<div
  class="transparent absolute border-l border-t pt-4 bottom-24 right-0 w-[400px] h-max max-h-[80vh] flex flex-col gap-2 overflow-hidden"
>
  <div class="flex flex-row items-center w-full justify-between px-4">
    <h3>Play Queue / {$playerQueue.queue.length}</h3>
    <button
      class="p-0 hover:text-crystal"
      on:click={() => ($showPlayQueue = false)}
    >
      <Icon icon="ph:x" width={18} height={18} />
    </button>
  </div>

  <section
    use:dndzone={{
      items: renderedList,
      flipDurationMs,
      dropTargetStyle: { outline: 'var(--color-crystal) solid 2px' },
      // transformDraggedElement,
    }}
    on:consider={handleConsider}
    on:finalize={handleSort}
    class="px-4 overflow-y-scroll border-t border-gray-600 py-2"
  >
    {#each renderedList as song, idx (song.id)}
      {@const isCurrent = $currentSong?.id === song.id}
      <div animate:flip={{ duration: flipDurationMs }}>
        <button
          on:click={() => playerQueue.selectSong(idx)}
          class="group draggable w-full py-2 gap-4 {isCurrent
            ? 'text-crystal'
            : ''} flex flex-row justify-between"
        >
          <h5
            class="flex-1 whitespace-nowrap overflow-hidden overflow-ellipsis text-left"
          >
            {song.title}
          </h5>
          <button class="p-0 text-text group-hover:text-crystal">
            <Icon icon="ph:x" width={18} height={18} />
          </button>
        </button>
      </div>
    {/each}
  </section>
</div>
