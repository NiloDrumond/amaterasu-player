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
  class="p-4 text-primary-bg rounded-full bg-crystal-primary hover:-translate-y-0.5 hover:text-black hover:bg-crystal-primary hover:drop-shadow-[0_0_10px_#DB48B9]"
  {...$$restProps}
  on:click
>
  <Icon {...iconProps} {icon} width={28} height={28} />
</button>
