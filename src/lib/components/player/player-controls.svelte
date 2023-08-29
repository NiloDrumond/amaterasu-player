<script lang="ts">
  import Icon from '@iconify/svelte';
  import { playerControls } from '$lib/stores/player';
  import { ndClient } from '$lib/navidrome/client';
  import type { PointerEventHandler } from 'svelte/elements';
  import { formatDuration } from '$lib/utils/format';

  let streamUrl: string | undefined;
  let paused = true;
  let duration = 0;
  let currentTime = 0;
  $: if ($playerControls.currentSong) {
    streamUrl = ndClient.getSongStreamUrl({
      songId: $playerControls.currentSong.id,
    });
  }
  let audio: HTMLAudioElement;
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

      currentTime = p * duration;
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
</script>

<audio
  src={streamUrl}
  bind:this={audio}
  bind:paused
  bind:duration
  bind:currentTime
  on:ended={() => {
    // TODO: HANDLE DRAG
    // currentTime = 0;
  }}
/>
<section class="flex flex-col justify-center items-center gap-2">
  <div class="flex flex-row items-center gap-6">
    <button class="hover:text-crystal p-0">
      <Icon icon="mingcute:shuffle-2-fill" width={20} height={20} />
    </button>
    <button class="hover:text-crystal p-0">
      <Icon
        icon="mingcute:skip-forward-fill"
        class="rotate-180"
        width={20}
        height={20}
      />
    </button>

    <button class="hover:text-crystal p-0" on:click={() => (paused = !paused)}>
      <Icon
        icon={paused
          ? 'mingcute:play-circle-fill'
          : 'mingcute:pause-circle-fill'}
        width={40}
        height={40}
      />
    </button>
    <button class="hover:text-crystal p-0">
      <Icon icon="mingcute:skip-forward-fill" width={20} height={20} />
    </button>
    <button class="hover:text-crystal p-0">
      <Icon icon="mingcute:repeat-fill" width={20} height={20} />
    </button>
  </div>
  <div class="flex flex-row gap-2 w-full items-center">
    <p class="w-[8%] text-right pr-2">
      {formatDuration(currentTime)}
    </p>
    <div
      class="flex-1 bg-slate-400 rounded-full h-2 dark:bg-gray-600 relative cursor-pointer select-none group"
      on:pointerdown={handlePointerDownProgress}
    >
      <div
        class=" h-2 rounded-full group-hover:bg-crystal {isDragging
          ? 'bg-crystal'
          : 'bg-slate-600 dark:bg-gray-100'}"
        style="width: {(currentTime / duration) * 100}%"
      ></div>
      <div
        class="absolute w-4 h-4 group-hover:bg-crystal rounded-full z-10 -top-1 -translate-x-[75%] {isDragging
          ? 'bg-crystal'
          : 'bg-slate-600 dark:bg-gray-100'}"
        style="left: calc({(currentTime / duration) * 100}% + 0.25rem);"
      />
    </div>
    <p class="w-[8%] pl-2 text-left">{formatDuration(duration)}</p>
  </div>
</section>
