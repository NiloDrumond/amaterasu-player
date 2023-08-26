<script lang="ts">
  import Icon, { type IconProps } from '@iconify/svelte';
  import type { HTMLButtonAttributes } from 'svelte/elements';
  interface $$Props extends HTMLButtonAttributes {
    icon: IconProps['icon'];
    iconProps?: Omit<IconProps, 'icon'>;
    size?: number;
  }
  const { icon, iconProps, size } = $$props;
  export const width: number = size || 20;
  export const height: number = size || 20;
</script>

<button {...$$restProps} on:click>
  <Icon {icon} {width} {height} {...iconProps} />
</button>
