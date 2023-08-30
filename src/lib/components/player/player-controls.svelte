<script lang="ts">
  import Icon from '@iconify/svelte';
  import type { PointerEventHandler } from 'svelte/elements';
  import { formatDuration } from '$lib/utils/format';
  import { audioPlayer, playerQueue } from '$lib/stores/player';
  import { createEventDispatcher } from 'svelte';

  const dispatch = createEventDispatcher<{ draggingtimeline: boolean }>();

  export let isDragging = false;
  let movedProgress = false;
  function handlePointerDownProgress(
    e: Parameters<PointerEventHandler<HTMLDivElement>>[0],
  ) {
    isDragging = true;
    dispatch('draggingtimeline', true);
    const div = e.currentTarget;

    function seek(e: PointerEvent) {
      const { left, width } = div.getBoundingClientRect();

      let p = (e.clientX - left) / width;
      if (p < 0) p = 0;
      if (p > 1) p = 1;

      $audioPlayer.currentTime = p * $audioPlayer.duration;
      movedProgress = true;
    }

    seek(e);

    window.addEventListener('pointermove', seek);

    window.addEventListener(
      'pointerup',
      () => {
        window.removeEventListener('pointermove', seek);
        if (!movedProgress) seek(e);
        movedProgress = false;
        isDragging = false;
        dispatch('draggingtimeline', false);
      },
      {
        once: true,
      },
    );
  }

  const PREVIOUS_SONG_THRESHOLD = 2;
  function handleGoBack() {
    if ($audioPlayer.currentTime < PREVIOUS_SONG_THRESHOLD) {
      playerQueue.previousSong();
    } else {
      $audioPlayer.currentTime = 0;
    }
  }
</script>

<section class="flex flex-col justify-between items-center gap-2 mb-1">
  <div class="flex flex-row items-center gap-6 h-12">
    <button class="hover:text-crystal p-0">
      <Icon icon="mingcute:shuffle-2-fill" width={20} height={20} />
    </button>
    <button class="hover:text-crystal p-0" on:click={handleGoBack}>
      <Icon
        icon="mingcute:skip-forward-fill"
        class="rotate-180"
        width={20}
        height={20}
      />
    </button>
    <button
      class="hover:text-crystal p-0"
      on:click={() => ($audioPlayer.paused = !$audioPlayer.paused)}
    >
      <Icon
        icon={$audioPlayer.paused
          ? 'mingcute:play-circle-fill'
          : 'mingcute:pause-circle-fill'}
        width={40}
        height={40}
      />
    </button>
    <button class="hover:text-crystal p-0" on:click={playerQueue.nextSong}>
      <Icon icon="mingcute:skip-forward-fill" width={20} height={20} />
    </button>
    <button class="hover:text-crystal p-0">
      <Icon icon="mingcute:repeat-fill" width={20} height={20} />
    </button>
  </div>
  <div class="flex flex-row gap-2 w-full items-center">
    <p class="w-12 text-right pr-2">
      {formatDuration($audioPlayer.currentTime)}
    </p>
    <div
      class="flex-1 bg-slate-400 rounded-full h- dark:bg-gray-600 relative cursor-pointer select-none group"
      on:pointerdown={handlePointerDownProgress}
    >
      <div
        class=" h-2 rounded-full group-hover:bg-crystal {isDragging
          ? 'bg-crystal'
          : 'bg-slate-600 dark:bg-gray-100'}"
        style="width: {($audioPlayer.currentTime / $audioPlayer.duration) *
          100}%"
      ></div>
      <div
        class="absolute w-4 h-4 group-hover:bg-crystal rounded-full z-10 -top-1 -translate-x-[75%] {isDragging
          ? 'bg-crystal'
          : 'bg-slate-600 dark:bg-gray-100'}"
        style="left: calc({($audioPlayer.currentTime / $audioPlayer.duration) *
          100}% + 0.25rem);"
      />
    </div>
    <p class="w-12 pl-2 text-left">
      {#if $audioPlayer.duration === 0}
        --:--
      {:else}
        {formatDuration($audioPlayer.duration)}
      {/if}
    </p>
  </div>
</section>
