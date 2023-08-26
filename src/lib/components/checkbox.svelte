<script lang="ts">
  import Icon from '@iconify/svelte';

  const { key } = $$props;
  const id = 'checkbox' + key;
  export let checked: boolean;
  export let onChange: (ev: Event) => void;
</script>

<div class="relative">
  <input
    type="checkbox"
    class="hidden checkbox peer"
    {id}
    {...$$restProps}
    on:change={onChange}
    bind:checked
  />
  <label
    for={id}
    class="w-5 h-5 border-2 rounded-md border-slate-300 cursor-pointer transition-colors flex items-center justify-center peer-checked:bg-crystal peer-checked:border-crystal"
    aria-hidden="true"
    ><Icon
      icon="mingcute:check-fill"
      class={`m-auto text-slate-800 translate-y-[1px] ${
        checked ? 'visible' : 'hidden'
      }`}
      width={14}
      height={14}
    /></label
  >
</div>
