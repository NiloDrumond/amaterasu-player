<script lang="ts">
  import Icon, { type IconProps } from '@iconify/svelte';
  import type { HTMLButtonAttributes } from 'svelte/elements';
  interface $$Props extends HTMLButtonAttributes {
    icon: IconProps['icon'];
    iconProps?: Omit<IconProps, 'icon'>;
  }
  const { icon, iconProps } = $$props;
</script>

<button
  class="p-4 rounded-full bg-purple-500 hover:bg-opacity-80"
  {...$$restProps}
  on:click
>
  <Icon {...iconProps} {icon} width={24} height={24} />
</button>
