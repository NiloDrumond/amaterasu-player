<script lang="ts">
  import Icon from '@iconify/svelte';
  import type { PointerEventHandler } from 'svelte/elements';
  import { playerQueue } from '$lib/stores/player-queue';
  import { showPlayQueue } from '$lib/stores/ui';
  import { audioPlayer } from '$lib/stores/audio';

  let isDragging = false;
  let movedProgress = false;
  function handlePointerDownProgress(
    e: Parameters<PointerEventHandler<HTMLDivElement>>[0],
  ) {
    isDragging = true;
    const div = e.currentTarget;

    function seek(e: PointerEvent) {
      const { left, width } = div.getBoundingClientRect();

      let p = (e.clientX - left) / width;
      if (p < 0) p = 0;
      if (p > 1) p = 1;

      $audioPlayer.volume = p;
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
      },
      {
        once: true,
      },
    );
  }
  let effectiveVolume: number;
  $: effectiveVolume = $audioPlayer.muted ? 0 : $audioPlayer.volume;
</script>

<section class="flex flex-col justify-between items-end gap-2 pr-2 mb-1">
  <div class="flex flex-row items-center gap-6 h-12">
    <button class="hover:text-crystal p-0">
      <Icon icon="ph:music-notes-plus-bold" width={20} height={20} />
    </button>
    <div class="flex flex-row items-center gap-1">
      <button
        class="hover:text-crystal p-0"
        on:click={() => ($showPlayQueue = !$showPlayQueue)}
      >
        <Icon icon="ph:list-bold" width={20} height={20} />
      </button>
      {$playerQueue.queue.length}
    </div>
    <button class="hover:text-crystal p-0" on:click={playerQueue.clearQueue}>
      <Icon icon="ph:x-bold" width={20} height={20} />
    </button>
  </div>
  <div class="flex flex-row gap-2 w-full max-w-[300px] items-center">
    <button
      class="hover:text-crystal p-0"
      on:click={audioPlayer.toggleMute}
    >
      <Icon
        icon={effectiveVolume > 0
          ? 'mingcute:volume-fill'
          : 'mingcute:volume-mute-fill'}
        width={18}
        height={18}
      />
    </button>
    <div
      class="flex-1 bg-slate-400 rounded-full h-1.5 dark:bg-gray-600 relative cursor-pointer select-none group"
      on:pointerdown={handlePointerDownProgress}
    >
      <div
        class="h-1.5 rounded-full group-hover:bg-crystal {isDragging
          ? 'bg-crystal'
          : 'bg-slate-600 dark:bg-gray-100'}"
        style="width: {effectiveVolume * 100}%"
      ></div>
      <div
        class="absolute opacity-0 w-3 h-3 group-hover:bg-crystal group-hover:opacity-100 rounded-full z-10 -top-1 -translate-x-[75%] {isDragging
          ? 'bg-crystal opacity-100'
          : 'bg-slate-600 dark:bg-gray-100 opacity-0'}"
        style="left: calc({effectiveVolume * 100}% + 0.25rem);"
      />
    </div>
  </div>
</section>
