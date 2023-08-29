<script lang="ts">
  import Icon from '@iconify/svelte';
</script>

<div
  class="sticky bottom-0 left-0 min-h-[6rem] h-24 bg-slate-200 bg-opacity-90 border-t border-slate-300 w-full grid grid-cols-3 dark:bg-gray-900 dark:bg-opacity-80 dark:border-gray-950 p-2"
>
  <section class="flex justify-start items-center gap-2">
    <div class="bg-red-500 w-10 h-10 rounded-full" />
    <div class="flex flex-col items-start">
      <h3>Title</h3>
      <h5>Album</h5>
      <h5>Artist</h5>
    </div>
  </section>
  <section class="flex flex-col justify-center items-center">
    <div class="flex flex-row items-center gap-2">
      <button class="hover:text-crystal">
        <Icon icon="mingcute:shuffle-2-line" width={20} height={20} />
      </button>
      <button class="hover:text-crystal">
        <Icon
          icon="mingcute:skip-forward-fill"
          class="rotate-180"
          width={20}
          height={20}
        />
      </button>
      <button class="hover:text-crystal">
        <Icon icon="mingcute:play-circle-fill" width={40} height={40} />
      </button>
      <button class="hover:text-crystal">
        <Icon icon="mingcute:skip-forward-fill" width={20} height={20} />
      </button>
      <button class="hover:text-crystal">
        <Icon icon="mingcute:repeat-line" width={20} height={20} />
      </button>
    </div>
    <div class="flex flex-row gap-2 w-full items-center">
      0:00
      <div
        class="w-full bg-slate-200 rounded-full h-2 dark:bg-gray-600 relative"
      >
        <div class="bg-crystal h-2 rounded-full" style="width: 45%"></div>
        <div
          class="absolute w-4 h-4 border-4 border-crystal rounded-full z-10 dark:bg-gray-600 -top-1"
          style="left: calc(45% - 0.5rem)"
        />
      </div>
      3:34
    </div>
  </section>
  <section class="flex justify-end items-center">
    Queue, Options, Favorite?
  </section>
</div>
