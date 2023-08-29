<script lang="ts">
  import Icon from '@iconify/svelte';
</script>

<div
  class="sticky bottom-0 left-0 h-24 bg-slate-200 bg-opacity-90 border-t border-slate-300 w-full grid grid-cols-3 dark:bg-gray-900 dark:bg-opacity-90 dark:border-gray-950 p-2"
>
  <section
    class="flex justify-start items-center gap-3 max-h-full overflow-y-hidden"
  >
    <div class="h-full aspect-square bg-slate-300 dark:bg-gray-700" />
    <div class="flex flex-col items-start">
      <h4>----</h4>
      <p>----</p>
    </div>
  </section>
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
      <button class="hover:text-crystal p-0">
        <Icon icon={'mingcute:play-circle-fill'} width={40} height={40} />
      </button>
      <button class="hover:text-crystal p-0">
        <Icon icon="mingcute:skip-forward-fill" width={20} height={20} />
      </button>
      <button class="hover:text-crystal p-0">
        <Icon icon="mingcute:repeat-fill" width={20} height={20} />
      </button>
    </div>
    <div class="flex flex-row gap-2 w-full items-center">
      <p class="w-[8%] text-right pr-2">--:--</p>
      <div
        class="flex-1 bg-slate-400 rounded-full h-2 dark:bg-gray-600 relative cursor-pointer select-none group"
      ></div>
      <p class="w-[8%] pl-2 text-left">--:--</p>
    </div>
  </section>
  <section class="flex justify-end items-center">
    Queue, Options, Favorite?, Volume
  </section>
</div>
