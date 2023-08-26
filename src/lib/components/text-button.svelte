<script lang="ts">
  import Icon, { type IconProps } from '@iconify/svelte';
  import type { HTMLButtonAttributes } from 'svelte/elements';
  export let label: string;
  interface $$Props extends HTMLButtonAttributes {
    label: string;
    icon: IconProps['icon'];
    iconProps?: Omit<IconProps, 'icon'>;
    size?: number;
  }
  const { icon, iconProps, size } = $$props;
  const width: number = size || 24;
  const height: number = size || 24;
</script>

<button {...$$restProps} class={`flex flex-row gap-2 ${$$props.class}`}>
  <Icon {...iconProps} {icon} {width} {height} />
  <p>{label}</p>
</button>
