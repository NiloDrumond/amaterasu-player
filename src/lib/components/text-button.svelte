<script lang="ts">
  import Icon, { type IconProps } from '@iconify/svelte';
  import type { HTMLButtonAttributes } from 'svelte/elements';
  export let label: string;
  interface $$Props extends HTMLButtonAttributes {
    label: string;
    icon: IconProps['icon'];
    iconProps?: Omit<IconProps, 'icon'>;
  }
  const { icon, iconProps } = $$props;
</script>

<button
  {...$$restProps}
  class={`flex flex-row gap-2 hover:text-white ${$$props.class}`}
>
  <Icon {...iconProps} {icon} width={24} height={24} />
  <p>{label}</p>
</button>
