<script lang="ts">
	import { cn } from '$lib/utils';
	import { Command as CommandPrimitive } from 'bits-ui';

	let { ref = $bindable(null), ...restProps }: CommandPrimitive.LoadingProps = $props();
</script>

<CommandPrimitive.Loading class={cn('px-2 py-1.5')} bind:ref {...restProps} />
