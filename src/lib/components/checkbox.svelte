<script lang="ts">
  const { key } = $$props;
  const id = 'checkbox' + key;
  export let checked: boolean;
  export let onChange: (ev: Event) => void;
</script>

<div class="relative">
  <input
    type="checkbox"
    class="hidden checkbox"
    {id}
    {...$$restProps}
    on:change={onChange}
    bind:checked
  />
  <label
    for={id}
    class="w-5 h-5 border-2 rounded-md border-primary block cursor-pointer transition-colors"
    aria-hidden="true"
  >
    <!-- SVG for the checkmark when the checkbox is checked -->
    <svg
      class="hidden w-full h-full text-white pointer-events-none"
      viewBox="0 0 24 24"
      fill="none"
      xmlns="http://www.w3.org/2000/svg"
    >
      <path
        d="M20 6L9 17L4 12"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
      ></path>
    </svg>
  </label>
</div>

<style>
  /* When the checkbox is checked, change the border and background colors */
  .checkbox:checked + label {
    @apply border-crystal-primary bg-crystal-primary;
  }
</style>
